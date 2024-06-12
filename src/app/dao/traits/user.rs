// ********************* import ********************* //
use async_trait::async_trait;

use super::{super::types::user::prelude::*, DataAccess};

// ********************* content ********************* //
#[async_trait]
pub trait UserDataAccess:
    DataAccess<
    DataModel = UserDataModel,
    DataAttr = UserAttr,
    FilterParam = UserFilterParam,
    CreateParam = UserCreateParam,
    UpdateParam = UserUpdateParam,
>
{
}
