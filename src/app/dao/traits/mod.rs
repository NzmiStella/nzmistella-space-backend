// ********************* mod ********************* //
pub mod user;

pub mod prelude {
    pub use super::{user::UserDataAccess, DataAccess};
}

// ********************* import ********************* //
use async_trait::async_trait;

use super::types::{OrderParam, PaginateParam};
use crate::app::common::prelude::AppResult;

// ********************* content ********************* //
#[async_trait]
pub trait DataAccess {
    type DataModel;
    type DataAttr;
    type FilterParam;
    type CreateParam;
    type UpdateParam;
    async fn count(&self, filter: Self::FilterParam) -> AppResult<u64>;
    // return the first one if multiple data found
    async fn get(&self, filter: Self::FilterParam) -> AppResult<Self::DataModel>;
    async fn list(
        &self,
        filter: Self::FilterParam,
        order: OrderParam<Self::DataAttr>,
        paginate: PaginateParam,
    ) -> AppResult<Vec<Self::DataModel>>;
    async fn create(&self, create_param: Self::CreateParam) -> AppResult<Self::DataModel>;
    async fn create_many(&self, create_param: Vec<Self::CreateParam>) -> AppResult<u64>;
    // return ResourceConflict error if multiple data found
    async fn update(
        &self,
        filter: Self::FilterParam,
        update_param: Self::UpdateParam,
    ) -> AppResult<()>;
    async fn update_all(
        &self,
        filter: Self::FilterParam,
        update_param: Self::UpdateParam,
    ) -> AppResult<u64>;
    // return ResourceConflict error if multiple data found
    async fn delete(&self, filter: Self::FilterParam) -> AppResult<()>;
    async fn delete_all(&self, filter: Self::FilterParam) -> AppResult<u64>;
}
