// ********************* import ********************* //
use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ColumnTrait, Condition, IntoActiveModel, IntoSimpleExpr, Set};
use sea_query::{IntoCondition, SimpleExpr};

use super::{
    super::{traits::user::UserDataAccess, types::user::prelude::*},
    DBConnProvider, DataAccessImpl,
};
use crate::app::db::prelude::{DatabaseConnection, UserActiveModel, UserColumn, UserEntity};

// ********************* content ********************* //
// params
impl IntoCondition for UserFilterParam {
    fn into_condition(self) -> Condition {
        let mut condition = Condition::all();
        if let Some(id) = self.id {
            condition = condition.add(UserColumn::Id.eq(id));
        }
        if let Some(username) = self.username {
            condition = condition.add(UserColumn::Username.eq(username));
        }
        if let Some(nickname) = self.nickname {
            condition = condition.add(UserColumn::Nickname.eq(nickname));
        }
        if let Some(group_type) = self.group_type {
            condition = condition.add(UserColumn::GroupType.eq(group_type));
        }
        if let Some(status_type) = self.status_type {
            condition = condition.add(UserColumn::StatusType.eq(status_type));
        }
        if let Some(name_search) = self.name_search {
            let name_search_cond = Condition::any()
                .add(UserColumn::Username.contains(&name_search))
                .add(UserColumn::Nickname.contains(&name_search));
            condition = condition.add(name_search_cond);
        }
        condition
    }
}

impl IntoActiveModel<UserActiveModel> for UserCreateParam {
    fn into_active_model(self) -> UserActiveModel {
        UserActiveModel {
            username: Set(self.username.clone()),
            nickname: Set(self.username),
            password: Set(self.password),
            email: Set(self.email),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<UserActiveModel> for UserUpdateParam {
    fn into_active_model(self) -> UserActiveModel {
        let mut active_model = <UserActiveModel as Default>::default();
        if let Some(nickname) = self.nickname {
            active_model.nickname = Set(nickname);
        }
        if let Some(password) = self.password {
            active_model.password = Set(password);
        }
        if let Some(email) = self.email {
            active_model.email = Set(email);
        }
        if let Some(avatar_url) = self.avatar_url {
            active_model.avatar_url = Set(avatar_url);
        }
        if let Some(signature) = self.signature {
            active_model.signature = Set(signature);
        }
        if let Some(group_type) = self.group_type {
            active_model.group_type = Set(group_type);
        }
        if let Some(status_type) = self.status_type {
            active_model.status_type = Set(status_type);
        }
        active_model
    }
}

impl IntoSimpleExpr for UserAttr {
    fn into_simple_expr(self) -> SimpleExpr {
        match self {
            UserAttr::Id => UserColumn::Id,
            UserAttr::Username => UserColumn::Username,
            UserAttr::Nickname => UserColumn::Nickname,
            UserAttr::CreateTime => UserColumn::CreateTime,
            UserAttr::UpdateTime => UserColumn::UpdateTime,
        }
        .into_simple_expr()
    }
}

// dao
pub struct UserDAO {
    db_conn: Arc<DatabaseConnection>,
}

impl UserDAO {
    pub fn new(db_conn: Arc<DatabaseConnection>) -> Self {
        Self { db_conn }
    }
}

impl DBConnProvider for UserDAO {
    fn db_conn(&self) -> &DatabaseConnection {
        &self.db_conn
    }
}

#[async_trait]
impl DataAccessImpl for UserDAO {
    type DataAttr = UserAttr;
    type FilterParam = UserFilterParam;
    type CreateParam = UserCreateParam;
    type UpdateParam = UserUpdateParam;
    type Model = UserDataModel;
    type Entity = UserEntity;
    type ActiveModel = UserActiveModel;
}

#[async_trait]
impl UserDataAccess for UserDAO {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{
        config::AppConfig,
        dao::{
            traits::prelude::DataAccess,
            types::{OrderParam, PaginateParam},
        },
        db::prelude::create_db_conn,
    };

    #[tokio::test]
    async fn test_user_dao() {
        // init
        let cfg = AppConfig::init("config/config_test.toml").unwrap();
        let db_conn = Arc::new(create_db_conn(&cfg.db).await.unwrap());
        let user_dao = UserDAO::new(db_conn);

        // delete all test data
        let test_filter = UserFilterParam {
            group_type: Some(0),
            ..Default::default()
        };
        <UserDAO as DataAccess>::delete_all(&user_dao, test_filter.clone())
            .await
            .unwrap();

        // test create success
        let create_param = UserCreateParam {
            username: "test_user".to_string(),
            password: "test".to_string(),
            email: "test@test_user.com".to_string(),
        };
        let user = <UserDAO as DataAccess>::create(&user_dao, create_param.clone())
            .await
            .unwrap();
        assert_eq!(user.username, "test_user");

        // test create failed(user already exists)
        let create_res = <UserDAO as DataAccess>::create(&user_dao, create_param.clone()).await;
        assert!(create_res.is_err());

        // test count
        let user_num = <UserDAO as DataAccess>::count(&user_dao, test_filter.clone())
            .await
            .unwrap();
        assert_eq!(user_num, 1);

        // test get
        let user_filter = UserFilterParam {
            username: Some("test_user".to_string()),
            ..Default::default()
        };
        let user = <UserDAO as DataAccess>::get(&user_dao, user_filter.clone())
            .await
            .unwrap();
        assert_eq!(user.email, "test@test_user.com");

        // test get failed(user not found)
        let user_not_found_filter = UserFilterParam {
            username: Some("test_user_not_found".to_string()),
            ..Default::default()
        };
        let get_res = <UserDAO as DataAccess>::get(&user_dao, user_not_found_filter.clone()).await;
        assert!(get_res.is_err());

        // test create_many
        let bulk_create_param = vec![
            UserCreateParam {
                username: "test_user1".to_string(),
                password: "test".to_string(),
                email: "test@test_user1.com".to_string(),
            },
            UserCreateParam {
                username: "test_user2".to_string(),
                password: "test".to_string(),
                email: "test@test_user2.com".to_string(),
            },
        ];
        let create_num = <UserDAO as DataAccess>::create_many(&user_dao, bulk_create_param)
            .await
            .unwrap();
        assert_eq!(create_num, 2);
        let user_num = <UserDAO as DataAccess>::count(&user_dao, test_filter.clone())
            .await
            .unwrap();
        assert_eq!(user_num, 3);

        // test list
        let user_list = <UserDAO as DataAccess>::list(
            &user_dao,
            test_filter.clone(),
            OrderParam::<UserAttr>::default(),
            PaginateParam::default(),
        )
        .await
        .unwrap();
        assert_eq!(user_list.len(), 3);
        assert_eq!(user_list[2].username, "test_user");

        // test update success
        let update_param = UserUpdateParam {
            nickname: Some("test_nickname".to_string()),
            ..Default::default()
        };
        <UserDAO as DataAccess>::update(&user_dao, user_filter.clone(), update_param.clone())
            .await
            .unwrap();
        let user = <UserDAO as DataAccess>::get(&user_dao, user_filter.clone());
        assert_eq!(user.await.unwrap().nickname, "test_nickname");

        // test update failed(multiple data found)
        let update_res =
            <UserDAO as DataAccess>::update(&user_dao, test_filter.clone(), update_param.clone())
                .await;
        assert!(update_res.is_err());

        // test update failed(user not found)
        let update_res = <UserDAO as DataAccess>::update(
            &user_dao,
            user_not_found_filter.clone(),
            update_param.clone(),
        )
        .await;
        assert!(update_res.is_err());

        // test update_all
        let update_num = <UserDAO as DataAccess>::update_all(
            &user_dao,
            test_filter.clone(),
            update_param.clone(),
        );
        assert_eq!(update_num.await.unwrap(), 3);
        let user_list = <UserDAO as DataAccess>::list(
            &user_dao,
            test_filter.clone(),
            OrderParam::<UserAttr>::default(),
            PaginateParam::default(),
        )
        .await
        .unwrap();
        for user in user_list {
            assert_eq!(user.nickname, "test_nickname");
        }

        // test delete success
        <UserDAO as DataAccess>::delete(&user_dao, user_filter)
            .await
            .unwrap();
        let user_num = <UserDAO as DataAccess>::count(&user_dao, test_filter.clone())
            .await
            .unwrap();
        assert_eq!(user_num, 2);

        // test delete failed(multiple data found)
        let delete_res = <UserDAO as DataAccess>::delete(&user_dao, test_filter.clone()).await;
        assert!(delete_res.is_err());

        // test delete failed(user not found)
        let delete_res =
            <UserDAO as DataAccess>::delete(&user_dao, user_not_found_filter.clone()).await;
        assert!(delete_res.is_err());

        // test delete_all
        let delete_num = <UserDAO as DataAccess>::delete_all(&user_dao, test_filter.clone())
            .await
            .unwrap();
        assert_eq!(delete_num, 2);
        let user_num = <UserDAO as DataAccess>::count(&user_dao, test_filter)
            .await
            .unwrap();
        assert_eq!(user_num, 0);
    }
}
