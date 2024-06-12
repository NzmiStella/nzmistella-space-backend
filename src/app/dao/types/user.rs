// ********************* mod ********************* //
pub mod prelude {
    pub use super::{
        Attr as UserAttr, CreateParam as UserCreateParam, DataModel as UserDataModel,
        FilterParam as UserFilterParam, UpdateParam as UserUpdateParam,
    };
}

// ********************* content ********************* //
pub type DataModel = crate::app::db::prelude::UserModel;

#[derive(Clone, Debug, Default)]
pub struct FilterParam {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub group_type: Option<i32>,
    pub status_type: Option<i32>,
    pub name_search: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CreateParam {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Clone, Debug, Default)]
pub struct UpdateParam {
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<Option<String>>,
    pub signature: Option<String>,
    pub group_type: Option<i32>,
    pub status_type: Option<i32>,
}

#[derive(Clone, Debug, Default)]
pub enum Attr {
    #[default]
    Id,
    Username,
    Nickname,
    CreateTime,
    UpdateTime,
}
