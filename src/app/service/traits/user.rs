use async_trait::async_trait;

use super::super::types::user::prelude::*;
use crate::app::common::prelude::AppResult;

#[async_trait]
pub trait UserServiceTrait {
    async fn register(&self, req_form: UserRegisterReqForm) -> AppResult<UserRegisterResForm>;
    async fn login(&self, req_form: UserLoginReqForm) -> AppResult<UserLoginResForm>;
    async fn availability(
        &self,
        req_form: UserAvailabilityReqForm,
    ) -> AppResult<UserAvailabilityResForm>;
    async fn search(
        &self,
        token: &str,
        req_form: UserSearchReqForm,
    ) -> AppResult<UserSearchResForm>;
    async fn find(&self, id: i32, token: &str) -> AppResult<UserFindResForm>;
    async fn change_password(
        &self,
        id: i32,
        token: &str,
        req_form: UserChangePasswordReqForm,
    ) -> AppResult<UserChangePasswordResForm>;
    async fn edit(
        &self,
        id: i32,
        token: &str,
        req_form: UserEditReqForm,
    ) -> AppResult<UserEditResForm>;
    async fn admin_search(
        &self,
        token: &str,
        req_form: UserAdminSearchReqForm,
    ) -> AppResult<UserAdminSearchResForm>;
    async fn admin_find(&self, id: i32, token: &str) -> AppResult<UserAdminGetResForm>;
    async fn admin_edit(
        &self,
        id: i32,
        token: &str,
        req_form: UserAdminEditReqForm,
    ) -> AppResult<UserAdminEditResForm>;
}
