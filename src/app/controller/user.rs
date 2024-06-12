// ********************* import ********************* //
use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    routing::{get, patch, post},
    Extension, Json, Router,
};
use garde::Validate;

use super::{BearerToken, HandlerAsyncSafe};
use crate::app::{
    common::prelude::*,
    service::{prelude::UserServiceTrait, types::user::prelude::*},
};

// ********************* content ********************* //
// router
pub fn public_router<U>(_: &U) -> Router
where
    U: UserServiceTrait + HandlerAsyncSafe,
{
    Router::new()
        .route("/register", post(register::<U>))
        .route("/login", post(login::<U>))
        .route("/availability", get(availability::<U>))
        .route("/search", get(search::<U>))
        .route("/:id", get(find::<U>).patch(edit::<U>))
        .route("/:id/password", patch(change_password::<U>))
}

pub fn admin_router<U>(_: &U) -> Router
where
    U: UserServiceTrait + HandlerAsyncSafe,
{
    Router::new()
        .route("/search", get(admin_search::<U>))
        .route("/:id", get(admin_find::<U>).patch(admin_edit::<U>))
}

// handler
async fn register<U>(
    Extension(user_service): Extension<Arc<U>>,
    Json(req_form): Json<UserRegisterReqForm>,
) -> AppResponse
where
    U: UserServiceTrait,
{
    req_form.validate(&()).wrap_with(
        || format!("Request form validation failed, form: {:?}", req_form),
        AppErrorKind::RequestParamInvalid,
    )?;
    user_service.register(req_form).await.into()
}

async fn login<U>(
    Extension(user_service): Extension<Arc<U>>,
    Json(req_form): Json<UserLoginReqForm>,
) -> AppResponse
where
    U: UserServiceTrait,
{
    req_form.validate(&()).wrap_with(
        || format!("Request form validation failed, form: {:?}", req_form),
        AppErrorKind::RequestParamInvalid,
    )?;
    user_service.login(req_form).await.into()
}

async fn availability<U>(
    Extension(user_service): Extension<Arc<U>>,
    Query(req_form): Query<UserAvailabilityReqForm>,
) -> AppResponse
where
    U: UserServiceTrait,
{
    req_form.validate(&()).wrap_with(
        || format!("Request form validation failed, form: {:?}", req_form),
        AppErrorKind::RequestParamInvalid,
    )?;
    user_service.availability(req_form).await.into()
}

async fn search<U>(
    Extension(user_service): Extension<Arc<U>>,
    BearerToken(token): BearerToken,
    Query(req_form): Query<UserSearchReqForm>,
) -> AppResponse
where
    U: UserServiceTrait,
{
    req_form.validate(&()).wrap_with(
        || format!("Request form validation failed, form: {:?}", req_form),
        AppErrorKind::RequestParamInvalid,
    )?;
    user_service.search(&token, req_form).await.into()
}

async fn find<U>(
    Extension(user_service): Extension<Arc<U>>,
    Path(id): Path<i32>,
    BearerToken(token): BearerToken,
) -> AppResponse
where
    U: UserServiceTrait,
{
    user_service.find(id, &token).await.into()
}

async fn change_password<U>(
    Extension(user_service): Extension<Arc<U>>,
    Path(id): Path<i32>,
    BearerToken(token): BearerToken,
    Json(req_form): Json<UserChangePasswordReqForm>,
) -> AppResponse
where
    U: UserServiceTrait,
{
    req_form.validate(&()).wrap_with(
        || format!("Request form validation failed, form: {:?}", req_form),
        AppErrorKind::RequestParamInvalid,
    )?;
    user_service
        .change_password(id, &token, req_form)
        .await
        .into()
}

async fn edit<U>(
    Extension(user_service): Extension<Arc<U>>,
    Path(id): Path<i32>,
    BearerToken(token): BearerToken,
    Json(req_form): Json<UserEditReqForm>,
) -> AppResponse
where
    U: UserServiceTrait,
{
    req_form.validate(&()).wrap_with(
        || format!("Request form validation failed, form: {:?}", req_form),
        AppErrorKind::RequestParamInvalid,
    )?;
    user_service.edit(id, &token, req_form).await.into()
}

async fn admin_search<U>(
    Extension(user_service): Extension<Arc<U>>,
    BearerToken(token): BearerToken,
    Query(req_form): Query<UserAdminSearchReqForm>,
) -> AppResponse
where
    U: UserServiceTrait,
{
    req_form.validate(&()).wrap_with(
        || format!("Request form validation failed, form: {:?}", req_form),
        AppErrorKind::RequestParamInvalid,
    )?;
    user_service.admin_search(&token, req_form).await.into()
}

async fn admin_find<U>(
    Extension(user_service): Extension<Arc<U>>,
    Path(id): Path<i32>,
    BearerToken(token): BearerToken,
) -> AppResponse
where
    U: UserServiceTrait,
{
    user_service.admin_find(id, &token).await.into()
}

async fn admin_edit<U>(
    Extension(user_service): Extension<Arc<U>>,
    Path(id): Path<i32>,
    BearerToken(token): BearerToken,
    Json(req_form): Json<UserAdminEditReqForm>,
) -> AppResponse
where
    U: UserServiceTrait,
{
    req_form.validate(&()).wrap_with(
        || format!("Request form validation failed, form: {:?}", req_form),
        AppErrorKind::RequestParamInvalid,
    )?;
    user_service.admin_edit(id, &token, req_form).await.into()
}
