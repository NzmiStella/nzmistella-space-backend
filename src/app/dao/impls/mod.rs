// ********************* mod ********************* //
pub mod user;

pub mod prelude {
    pub use super::{user::UserDAO, DataAccessImpl};
}

// ********************* import ********************* //
use std::fmt::Debug;

use async_trait::async_trait;
use sea_orm::{
    prelude::*, FromQueryResult, IntoActiveModel, IntoSimpleExpr, Order, QueryOrder,
    TransactionTrait,
};
use sea_query::IntoCondition;

use super::{
    traits::DataAccess,
    types::{OrderParam, PaginateParam},
};
use crate::app::{common::prelude::*, db::prelude::DBConnProvider};

// ********************* content ********************* //
impl<T> OrderParam<T> {
    fn order(&self) -> Order {
        match self.ascending {
            true => Order::Asc,
            false => Order::Desc,
        }
    }
}

#[async_trait]
pub trait DataAccessImpl {
    // DataAccess
    type DataAttr: IntoSimpleExpr + Clone + Send;
    type FilterParam: IntoCondition + Clone + Debug + Send;
    type CreateParam: IntoActiveModel<Self::ActiveModel> + Send;
    type UpdateParam: IntoActiveModel<Self::ActiveModel> + Send;
    // Sea-orm
    type Model: ModelTrait<Entity = Self::Entity>
        + IntoActiveModel<Self::ActiveModel>
        + FromQueryResult
        + Sync;
    type Entity: EntityTrait<Model = Self::Model>;
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity> + ActiveModelBehavior + Send;

    async fn count<C: ConnectionTrait>(
        &self,
        db_conn: &C,
        filter: Self::FilterParam,
    ) -> AppResult<u64> {
        Self::Entity::find()
            .filter(filter)
            .count(db_conn)
            .await
            .with_err_kind(AppErrorKind::DBOperationError)
    }

    async fn get<C: ConnectionTrait>(
        &self,
        db_conn: &C,
        filter: Self::FilterParam,
    ) -> AppResult<Self::Model> {
        Self::Entity::find()
            .filter(filter.clone())
            .one(db_conn)
            .await
            .with_err_kind(AppErrorKind::DBOperationError)
            .and_then(|op| {
                op.ok_or(AppError::new(
                    format!("Data not found, filter: {:?}", filter),
                    AppErrorKind::ResourceNotFound,
                ))
            })
    }

    async fn list<C: ConnectionTrait>(
        &self,
        db_conn: &C,
        filter: Self::FilterParam,
        order: OrderParam<Self::DataAttr>,
        paginate: PaginateParam,
    ) -> AppResult<Vec<Self::Model>> {
        Self::Entity::find()
            .filter(filter)
            .order_by(order.clone().by, order.order())
            .paginate(db_conn, paginate.page_size)
            .fetch_page(paginate.page_num.saturating_sub(1))
            .await
            .with_err_kind(AppErrorKind::DBOperationError)
    }

    async fn create<C: ConnectionTrait>(
        &self,
        db_conn: &C,
        create_param: Self::CreateParam,
    ) -> AppResult<Self::Model> {
        create_param
            .into_active_model()
            .insert(db_conn)
            .await
            .with_err_kind(AppErrorKind::DBOperationError)
    }

    async fn create_many<C: ConnectionTrait>(
        &self,
        db_conn: &C,
        create_param: Vec<Self::CreateParam>,
    ) -> AppResult<u64> {
        let len = create_param.len() as u64;
        Self::Entity::insert_many(
            create_param
                .into_iter()
                .map(<Self::CreateParam>::into_active_model),
        )
        .exec(db_conn)
        .await
        .map(|_| len)
        .with_err_kind(AppErrorKind::DBOperationError)
    }

    async fn update<C: ConnectionTrait + TransactionTrait>(
        &self,
        db_conn: &C,
        filter: Self::FilterParam,
        update_param: Self::UpdateParam,
    ) -> AppResult<()> {
        let txn = db_conn.begin().await.wrap(
            "Failed to start db transaction.",
            AppErrorKind::DBOperationError,
        )?;
        let count = self.count(&txn, filter.clone()).await?;
        match count {
            0 => {
                return Err(AppError::new(
                    format!("Data not found, filter: {:?}", filter),
                    AppErrorKind::ResourceNotFound,
                ));
            }
            1 => {}
            _ => {
                return Err(AppError::new(
                    format!(
                        "Multiple data found, count: {:?}, filter: {:?}",
                        count, filter
                    ),
                    AppErrorKind::ResourceConflict,
                ));
            }
        }
        self.update_all(&txn, filter, update_param).await?;
        txn.commit().await.wrap(
            "Failed to commit db transaction.",
            AppErrorKind::DBOperationError,
        )
    }

    async fn update_all<C: ConnectionTrait>(
        &self,
        db_conn: &C,
        filter: Self::FilterParam,
        update_param: Self::UpdateParam,
    ) -> AppResult<u64> {
        Self::Entity::update_many()
            .set(update_param.into_active_model())
            .filter(filter)
            .exec(db_conn)
            .await
            .map(|update_res| update_res.rows_affected)
            .with_err_kind(AppErrorKind::DBOperationError)
    }

    async fn delete<C: ConnectionTrait + TransactionTrait>(
        &self,
        db_conn: &C,
        filter: Self::FilterParam,
    ) -> AppResult<()> {
        let txn = db_conn.begin().await.wrap(
            "Failed to start db transaction.",
            AppErrorKind::DBOperationError,
        )?;
        let count = self.count(&txn, filter.clone()).await?;
        match count {
            0 => {
                return Err(AppError::new(
                    format!("Data not found, filter: {:?}", filter),
                    AppErrorKind::ResourceNotFound,
                ));
            }
            1 => {}
            _ => {
                return Err(AppError::new(
                    format!(
                        "Multiple data found, count: {:?}, filter: {:?}",
                        count, filter
                    ),
                    AppErrorKind::ResourceConflict,
                ));
            }
        }
        self.delete_all(&txn, filter).await?;
        txn.commit().await.wrap(
            "Failed to commit db transaction.",
            AppErrorKind::DBOperationError,
        )
    }

    async fn delete_all<C: ConnectionTrait>(
        &self,
        db_conn: &C,
        filter: Self::FilterParam,
    ) -> AppResult<u64> {
        Self::Entity::delete_many()
            .filter(filter)
            .exec(db_conn)
            .await
            .map(|delete_res| delete_res.rows_affected)
            .with_err_kind(AppErrorKind::DBOperationError)
    }
}

#[async_trait]
impl<T: DataAccessImpl + DBConnProvider + Sync> DataAccess for T {
    type DataModel = T::Model;
    type DataAttr = T::DataAttr;
    type FilterParam = T::FilterParam;
    type CreateParam = T::CreateParam;
    type UpdateParam = T::UpdateParam;

    async fn count(&self, filter: Self::FilterParam) -> AppResult<u64> {
        <Self as DataAccessImpl>::count(self, self.db_conn(), filter).await
    }

    async fn get(&self, filter: Self::FilterParam) -> AppResult<Self::DataModel> {
        <Self as DataAccessImpl>::get(self, self.db_conn(), filter).await
    }

    async fn list(
        &self,
        filter: Self::FilterParam,
        order: OrderParam<Self::DataAttr>,
        paginate: PaginateParam,
    ) -> AppResult<Vec<Self::DataModel>> {
        <Self as DataAccessImpl>::list(self, self.db_conn(), filter, order, paginate).await
    }

    async fn create(&self, create_param: Self::CreateParam) -> AppResult<Self::DataModel> {
        <Self as DataAccessImpl>::create(self, self.db_conn(), create_param).await
    }

    async fn create_many(&self, create_param: Vec<Self::CreateParam>) -> AppResult<u64> {
        <Self as DataAccessImpl>::create_many(self, self.db_conn(), create_param).await
    }

    async fn update(
        &self,
        filter: Self::FilterParam,
        update_param: Self::UpdateParam,
    ) -> AppResult<()> {
        <Self as DataAccessImpl>::update(self, self.db_conn(), filter, update_param).await
    }

    async fn update_all(
        &self,
        filter: Self::FilterParam,
        update_param: Self::UpdateParam,
    ) -> AppResult<u64> {
        <Self as DataAccessImpl>::update_all(self, self.db_conn(), filter, update_param).await
    }

    async fn delete(&self, filter: Self::FilterParam) -> AppResult<()> {
        <Self as DataAccessImpl>::delete(self, self.db_conn(), filter).await
    }

    async fn delete_all(&self, filter: Self::FilterParam) -> AppResult<u64> {
        <Self as DataAccessImpl>::delete_all(self, self.db_conn(), filter).await
    }
}
