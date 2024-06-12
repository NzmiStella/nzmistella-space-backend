// ********************* import ********************* //
use std::sync::Arc;

use async_trait::async_trait;

use super::super::{traits::user::UserServiceTrait, types::user::prelude::*};
use crate::app::{
    common::prelude::*,
    dao::{
        prelude::{OrderParam, PaginateParam, UserDataAccess},
        types::user::prelude::*,
    },
    utils::prelude::{CryptoUtilsTrait, Page, TokenUtilsTrait},
};

// ********************* content ********************* //
impl From<UserDataModel> for UserInfo {
    fn from(model: UserDataModel) -> Self {
        Self {
            id: model.id,
            username: model.username,
            nickname: model.nickname,
            email: model.email,
            avatar_url: model.avatar_url,
            signature: model.signature,
            group_type: model.group_type,
            status_type: model.status_type,
        }
    }
}

pub struct UserService<D, C, T>
where
    D: UserDataAccess + Sync + Send,
    C: CryptoUtilsTrait + Sync + Send,
    T: TokenUtilsTrait + Sync + Send,
{
    pub user_dao: Arc<D>,
    pub crypto_utils: Arc<C>,
    pub token_utils: Arc<T>,
}

impl<D, C, T> UserService<D, C, T>
where
    D: UserDataAccess + Sync + Send,
    C: CryptoUtilsTrait + Sync + Send,
    T: TokenUtilsTrait + Sync + Send,
{
    pub fn new(user_dao: Arc<D>, crypto_utils: Arc<C>, token_utils: Arc<T>) -> Self {
        Self {
            user_dao,
            crypto_utils,
            token_utils,
        }
    }

    async fn search_inner<S: ToString + Sync>(
        &self,
        token: &str,
        filter: UserFilterParam,
        paginate: PaginateParam,
        allow_group: &[S],
    ) -> AppResult<UserSearchResForm> {
        self.token_utils.verify_token(token, allow_group).await?;
        let record_total = self.user_dao.count(filter.clone()).await?;
        let model_infos = self
            .user_dao
            .list(filter, OrderParam::<UserAttr>::default(), paginate.clone())
            .await?
            .into_iter()
            .map(|model| model.into())
            .collect();
        let page = Page::new(
            paginate.page_num,
            paginate.page_size,
            record_total,
            model_infos,
        )
        .wrap(
            "Invalid pagination parameters",
            AppErrorKind::RequestParamInvalid,
        )?;
        Ok(page)
    }

    async fn find_inner<S: ToString + Sync>(
        &self,
        token: &str,
        filter_param: UserFilterParam,
        allow_group: &[S],
    ) -> AppResult<UserFindResForm> {
        self.token_utils.verify_token(token, allow_group).await?;
        let user_model = self.user_dao.get(filter_param).await?;
        Ok(UserFindResForm {
            user_info: user_model.into(),
        })
    }
}

#[async_trait]
impl<D, C, T> UserServiceTrait for UserService<D, C, T>
where
    D: UserDataAccess + Sync + Send,
    C: CryptoUtilsTrait + Sync + Send,
    T: TokenUtilsTrait + Sync + Send,
{
    async fn register(&self, req_form: UserRegisterReqForm) -> AppResult<UserRegisterResForm> {
        let cnt = self
            .user_dao
            .count(UserFilterParam {
                username: Some(req_form.username.clone()),
                ..Default::default()
            })
            .await?;
        if cnt > 0 {
            return Err(AppError::new(
                format!("Username '{}' already exists.", req_form.username),
                AppErrorKind::UsernameConflict,
            ));
        }

        let user_model = self
            .user_dao
            .create(UserCreateParam {
                username: req_form.username,
                password: self.crypto_utils.hash(&req_form.password)?,
                email: req_form.email,
            })
            .await?;
        let token = self
            .token_utils
            .generate_token(user_model.id, user_model.group_type, 3600 * 24 * 7)
            .await?;
        Ok(UserRegisterResForm {
            user_info: user_model.into(),
            token,
        })
    }

    async fn login(&self, req_form: UserLoginReqForm) -> AppResult<UserLoginResForm> {
        let user_model = self
            .user_dao
            .get(UserFilterParam {
                username: Some(req_form.username),
                ..Default::default()
            })
            .await?;
        self.crypto_utils
            .verify(&req_form.password, &user_model.password)?;
        let token = self
            .token_utils
            .generate_token(user_model.id, user_model.group_type, 3600 * 24 * 7)
            .await?;
        Ok(UserLoginResForm {
            user_info: user_model.into(),
            token,
        })
    }

    async fn availability(
        &self,
        req_form: UserAvailabilityReqForm,
    ) -> AppResult<UserAvailabilityResForm> {
        let cnt = self
            .user_dao
            .count(UserFilterParam {
                username: Some(req_form.username),
                ..Default::default()
            })
            .await?;
        Ok(UserAvailabilityResForm {
            available: cnt == 0,
        })
    }

    async fn search(
        &self,
        token: &str,
        req_form: UserSearchReqForm,
    ) -> AppResult<UserSearchResForm> {
        let filter = UserFilterParam {
            name_search: req_form.name_search,
            status_type: Some(1),
            ..Default::default()
        };
        let paginate = PaginateParam {
            page_num: req_form.page_num,
            page_size: req_form.page_size,
        };
        self.search_inner(token, filter, paginate, &[0, 1]).await
    }

    async fn find(&self, id: i32, token: &str) -> AppResult<UserFindResForm> {
        let filter = UserFilterParam {
            id: Some(id),
            status_type: Some(1),
            ..Default::default()
        };
        self.find_inner(token, filter, &[0, 1]).await
    }

    async fn change_password(
        &self,
        id: i32,
        token: &str,
        req_form: UserChangePasswordReqForm,
    ) -> AppResult<UserChangePasswordResForm> {
        self.token_utils
            .verify_token(token, &[0, 1])
            .await
            .and_then(|claims| {
                if claims.user_id != id {
                    return Err(AppError::new(
                        "You can change only your own password",
                        AppErrorKind::PermissionDenied,
                    ));
                }
                Ok(claims)
            })?;
        let filter = UserFilterParam {
            id: Some(id),
            ..Default::default()
        };
        let user_model = self.user_dao.get(filter.clone()).await?;
        self.crypto_utils
            .verify(&req_form.old_password, &user_model.password)?;
        self.user_dao
            .update(
                filter,
                UserUpdateParam {
                    password: Some(self.crypto_utils.hash(&req_form.new_password)?),
                    ..Default::default()
                },
            )
            .await?;
        self.token_utils.invalidate_token(id, 3600 * 24 * 7).await?;
        Ok(UserChangePasswordResForm)
    }

    async fn edit(
        &self,
        id: i32,
        token: &str,
        req_form: UserEditReqForm,
    ) -> AppResult<UserEditResForm> {
        self.token_utils
            .verify_token(token, &[0, 1])
            .await
            .and_then(|claims| {
                if claims.user_id != id {
                    return Err(AppError::new(
                        "You can change only your own info",
                        AppErrorKind::PermissionDenied,
                    ));
                }
                Ok(claims)
            })?;
        let filter_param = UserFilterParam {
            id: Some(id),
            ..Default::default()
        };
        self.user_dao
            .update(
                filter_param.clone(),
                UserUpdateParam {
                    nickname: req_form.nickname,
                    email: req_form.email,
                    signature: req_form.signature,
                    avatar_url: req_form.avatar_url.map(Some),
                    ..Default::default()
                },
            )
            .await?;
        let user_model = self.user_dao.get(filter_param).await?;
        Ok(UserEditResForm {
            user_info: user_model.into(),
        })
    }

    async fn admin_search(
        &self,
        token: &str,
        req_form: UserAdminSearchReqForm,
    ) -> AppResult<UserAdminSearchResForm> {
        let filter = UserFilterParam {
            name_search: req_form.name_search,
            group_type: req_form.group_type,
            status_type: req_form.status_type,
            ..Default::default()
        };
        let paginate = PaginateParam {
            page_num: req_form.page_num,
            page_size: req_form.page_size,
        };
        self.search_inner(token, filter, paginate, &[1]).await
    }

    async fn admin_find(&self, id: i32, token: &str) -> AppResult<UserAdminGetResForm> {
        let filter = UserFilterParam {
            id: Some(id),
            ..Default::default()
        };
        self.find_inner(token, filter, &[1]).await
    }

    async fn admin_edit(
        &self,
        id: i32,
        token: &str,
        req_form: UserAdminEditReqForm,
    ) -> AppResult<UserAdminEditResForm> {
        self.token_utils.verify_token(token, &[1]).await?;
        let filter_param = UserFilterParam {
            id: Some(id),
            ..Default::default()
        };
        self.user_dao
            .update(
                filter_param.clone(),
                UserUpdateParam {
                    group_type: req_form.group_type,
                    status_type: req_form.status_type,
                    ..Default::default()
                },
            )
            .await?;
        let user_model = self.user_dao.get(filter_param).await?;
        Ok(UserAdminEditResForm {
            user_info: user_model.into(),
        })
    }
}
