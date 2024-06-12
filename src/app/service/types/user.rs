// ********************* mod ********************* //
pub mod prelude {
    pub use super::{
        UserAdminEditReqForm, UserAdminEditResForm, UserAdminGetResForm, UserAdminSearchReqForm,
        UserAdminSearchResForm, UserAvailabilityReqForm, UserAvailabilityResForm,
        UserChangePasswordReqForm, UserChangePasswordResForm, UserEditReqForm, UserEditResForm,
        UserFindResForm, UserInfo, UserLoginReqForm, UserLoginResForm, UserRegisterReqForm,
        UserRegisterResForm, UserSearchReqForm, UserSearchResForm,
    };
}

// ********************* import ********************* //
use garde::Validate;
use serde::{Deserialize, Serialize};

use super::{BASIC_ASCII_RE, BASIC_UNICODE_RE};
use crate::app::utils::prelude::Page;

// ********************* content ********************* //
const NAME_MIN_LEN: usize = 5;
const NAME_MAX_LEN: usize = 16;
const PWD_MIN_LEN: usize = 5;
const PWD_MAX_LEN: usize = 16;
const NICKNAME_MIN_LEN: usize = 1;
const NICKNAME_MAX_LEN: usize = 16;
const SIGNATURE_MIN_LEN: usize = 1;
const SIGNATURE_MAX_LEN: usize = 512;
const GROUP_TYPE_MIN: i32 = 0;
const GROUP_TYPE_MAX: i32 = 1;
const STATUS_TYPE_MIN: i32 = 0;
const STATUS_TYPE_MAX: i32 = 3;
const NAME_SEARCH_MIN_LEN: usize = 1;
const NAME_SEARCH_MAX_LEN: usize = 16;

fn default_page_size() -> u64 {
    10
}

fn default_page_num() -> u64 {
    1
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub email: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    pub signature: String,
    #[serde(rename = "groupType")]
    pub group_type: i32,
    #[serde(rename = "statusType")]
    pub status_type: i32,
}

// register
#[derive(Debug, Deserialize, Validate)]
pub struct UserRegisterReqForm {
    #[garde(pattern(BASIC_ASCII_RE), length(min = NAME_MIN_LEN, max = NAME_MAX_LEN))]
    pub username: String,
    #[garde(pattern(BASIC_ASCII_RE), length(min = PWD_MIN_LEN, max = PWD_MAX_LEN))]
    pub password: String,
    #[garde(email)]
    pub email: String,
}
#[derive(Debug, Serialize)]
pub struct UserRegisterResForm {
    #[serde(rename = "userInfo")]
    pub user_info: UserInfo,
    pub token: String,
}

// login
#[derive(Debug, Deserialize, Validate)]
pub struct UserLoginReqForm {
    #[garde(pattern(BASIC_ASCII_RE), length(min = NAME_MIN_LEN, max = NAME_MAX_LEN))]
    pub username: String,
    #[garde(pattern(BASIC_ASCII_RE), length(min = PWD_MIN_LEN, max = PWD_MAX_LEN))]
    pub password: String,
}
pub type UserLoginResForm = UserRegisterResForm;

// availability
#[derive(Debug, Deserialize, Validate)]
pub struct UserAvailabilityReqForm {
    #[garde(pattern(BASIC_ASCII_RE), length(min = NAME_MIN_LEN, max = NAME_MAX_LEN))]
    pub username: String,
}
#[derive(Debug, Serialize)]
pub struct UserAvailabilityResForm {
    pub available: bool,
}

// search
#[derive(Debug, Deserialize, Validate)]
pub struct UserSearchReqForm {
    #[serde(rename = "nameSearch")]
    #[garde(pattern(BASIC_UNICODE_RE), length(min = NAME_SEARCH_MIN_LEN, max = NAME_SEARCH_MAX_LEN))]
    pub name_search: Option<String>,
    #[serde(rename = "pageNum", default = "default_page_num")]
    #[garde(range(min = 1))]
    pub page_num: u64,
    #[serde(rename = "pageSize", default = "default_page_size")]
    #[garde(range(min = 1))]
    pub page_size: u64,
}
pub type UserSearchResForm = Page<UserInfo>;

// get
#[derive(Debug, Serialize)]
pub struct UserFindResForm {
    #[serde(rename = "userInfo")]
    pub user_info: UserInfo,
}

// change password
#[derive(Debug, Deserialize, Validate)]
pub struct UserChangePasswordReqForm {
    #[serde(rename = "oldPassword")]
    #[garde(pattern(BASIC_ASCII_RE), length(min = PWD_MIN_LEN, max = PWD_MAX_LEN))]
    pub old_password: String,
    #[serde(rename = "newPassword")]
    #[garde(pattern(BASIC_ASCII_RE), length(min = PWD_MIN_LEN, max = PWD_MAX_LEN))]
    pub new_password: String,
}

#[derive(Serialize)]
pub struct UserChangePasswordResForm;

// edit
#[derive(Debug, Deserialize, Validate)]
pub struct UserEditReqForm {
    #[garde(pattern(BASIC_UNICODE_RE), length(min = NICKNAME_MIN_LEN, max = NICKNAME_MAX_LEN))]
    pub nickname: Option<String>,
    #[garde(email)]
    pub email: Option<String>,
    #[garde(length(min = SIGNATURE_MIN_LEN, max = SIGNATURE_MAX_LEN))]
    pub signature: Option<String>,
    #[serde(rename = "avatarUrl")]
    #[garde(url)]
    pub avatar_url: Option<String>,
}
pub type UserEditResForm = UserFindResForm;

// admin search
#[derive(Debug, Deserialize, Validate)]
pub struct UserAdminSearchReqForm {
    #[serde(rename = "nameSearch")]
    #[garde(pattern(BASIC_UNICODE_RE), length(min = NAME_SEARCH_MIN_LEN, max = NAME_SEARCH_MAX_LEN))]
    pub name_search: Option<String>,
    #[serde(rename = "groupType")]
    #[garde(range(min = GROUP_TYPE_MIN, max = GROUP_TYPE_MAX))]
    pub group_type: Option<i32>,
    #[serde(rename = "statusType")]
    #[garde(range(min = STATUS_TYPE_MIN, max = STATUS_TYPE_MAX))]
    pub status_type: Option<i32>,
    #[serde(rename = "pageNum", default = "default_page_num")]
    #[garde(range(min = 1))]
    pub page_num: u64,
    #[serde(rename = "pageSize", default = "default_page_size")]
    #[garde(range(min = 1))]
    pub page_size: u64,
}
pub type UserAdminSearchResForm = Page<UserInfo>;

// admin get
pub type UserAdminGetResForm = UserFindResForm;

// admin edit
#[derive(Debug, Deserialize, Validate)]
pub struct UserAdminEditReqForm {
    #[serde(rename = "groupType")]
    #[garde(range(min = GROUP_TYPE_MIN, max = GROUP_TYPE_MAX))]
    pub group_type: Option<i32>,
    #[serde(rename = "statusType")]
    #[garde(range(min = STATUS_TYPE_MIN, max = STATUS_TYPE_MAX))]
    pub status_type: Option<i32>,
}
pub type UserAdminEditResForm = UserEditResForm;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_inputs() {
        let forms = vec![
            UserRegisterReqForm {
                username: "valid_user".to_string(),
                password: "valid_pswd".to_string(),
                email: "email@example.com".to_string(),
            },
            UserRegisterReqForm {
                username: "valid_user_2".to_string(),
                password: "valid_pswd_2".to_string(),
                email: "email_2@example.com".to_string(),
            },
        ];

        for form in forms {
            assert!(form.validate(&()).is_ok());
        }
    }

    #[test]
    fn test_invalid_inputs() {
        let forms = vec![
            UserRegisterReqForm {
                username: "s".to_string(), // 太短
                password: "short".to_string(),
                email: "bademail".to_string(), // 错误的邮箱格式
            },
            UserRegisterReqForm {
                username: "!invalid*chars".to_string(),       // 包含非法字符
                password: "12345678901234567890".to_string(), // 太长
                email: "another@bademail".to_string(),
            },
        ];

        for form in forms {
            assert!(form.validate(&()).is_err());
        }
    }
}
