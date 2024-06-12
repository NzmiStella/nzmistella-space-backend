// ********************* import ********************* //
use std::{
    backtrace::Backtrace,
    fmt::{Debug, Display},
};

use anyhow::anyhow;
use http::StatusCode;
use strum::{EnumMessage, EnumProperty};

// ********************* content ********************* //
#[derive(Debug, Default, EnumMessage, EnumProperty)]
pub enum AppErrorKind {
    #[strum(
        message = "请求参数无效",
        props(http_code = "400", app_code = "-40000")
    )]
    RequestParamInvalid,
    #[strum(
        message = "缺少必须的请求参数",
        props(http_code = "400", app_code = "-40001")
    )]
    RequestParamMissing,
    #[strum(
        message = "缺少访问凭证",
        props(http_code = "401", app_code = "-40100")
    )]
    MissingCredential,
    #[strum(
        message = "不合法的凭证",
        props(http_code = "401", app_code = "-40101")
    )]
    MalformedCredential,
    #[strum(
        message = "已失效的凭证",
        props(http_code = "401", app_code = "-40102")
    )]
    InvalidCredential,
    #[strum(
        message = "访问权限不足",
        props(http_code = "403", app_code = "-40300")
    )]
    PermissionDenied,
    #[strum(
        message = "请求资源不存在",
        props(http_code = "404", app_code = "-40400")
    )]
    ResourceNotFound,
    #[strum(
        message = "请求资源不唯一",
        props(http_code = "409", app_code = "-40900")
    )]
    ResourceConflict,
    #[strum(
        message = "用户名已存在",
        props(http_code = "409", app_code = "-40901")
    )]
    UsernameConflict,
    #[strum(
        message = "服务端内部错误",
        props(http_code = "500", app_code = "-50000")
    )]
    #[default]
    InternalError,
    #[strum(
        message = "数据库操作错误",
        props(http_code = "500", app_code = "-50001")
    )]
    DBOperationError,
    #[strum(
        message = "缓存操作错误",
        props(http_code = "500", app_code = "-50002")
    )]
    CacheOperationError,
    #[strum(
        message = "配置参数错误",
        props(http_code = "500", app_code = "-50003")
    )]
    ConfigurationError,
    #[strum(
        message = "未实现的功能",
        props(http_code = "501", app_code = "-50100")
    )]
    NotImplemented,
}

impl AppErrorKind {
    pub fn message(&self) -> &'static str {
        self.get_message().unwrap_or_else(|| {
            panic!(
                "Failed to extract attribute 'message', error kind: \n{:#?}",
                self
            )
        })
    }

    pub fn http_code(&self) -> StatusCode {
        StatusCode::from_bytes(
            self.get_str("http_code")
                .unwrap_or_else(|| {
                    panic!(
                        "Failed to extract attribute 'http_code', error kind: \n{:#?}",
                        self
                    )
                })
                .as_bytes(),
        )
        .unwrap_or_else(|_| {
            panic!(
                "Failed to parse http::StatusCode, error kind: \n{:#?}",
                self
            )
        })
    }

    pub fn app_code(&self) -> i32 {
        self.get_str("app_code")
            .unwrap_or_else(|| {
                panic!(
                    "Failed to extract attribute 'app_code', error kind: \n{:#?}",
                    self
                )
            })
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Failed to parse app_code, error kind: \n{:#?}", self))
    }
}

pub struct AppError {
    pub kind: AppErrorKind,
    pub cause: anyhow::Error,
}

impl AppError {
    pub fn new<C>(context: C, kind: AppErrorKind) -> Self
    where
        C: Debug + Display + Send + Sync + 'static,
    {
        Self {
            kind,
            cause: anyhow!(context),
        }
    }
    pub fn context<C>(self, context: C) -> Self
    where
        C: Display + Send + Sync + 'static,
    {
        Self {
            kind: self.kind,
            cause: self.cause.context(context),
        }
    }
    pub fn with_context<C, F>(self, context: F) -> Self
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        Self {
            kind: self.kind,
            cause: self.cause.context(context()),
        }
    }
    pub fn message(&self) -> &'static str {
        self.kind.message()
    }
    pub fn http_code(&self) -> StatusCode {
        self.kind.http_code()
    }
    pub fn app_code(&self) -> i32 {
        self.kind.app_code()
    }
    pub fn backtrace(&self) -> &Backtrace {
        self.cause.backtrace()
    }
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\n{:?}", self.message(), &self.cause)
    }
}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\n{}", self.message(), &self.cause)
    }
}
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.source()
    }
}

pub trait IntoAppError {
    fn with_err_kind(self, err_kind: AppErrorKind) -> AppError;
}

impl<E> IntoAppError for E
where
    E: Into<anyhow::Error>,
{
    fn with_err_kind(self, err_kind: AppErrorKind) -> AppError {
        AppError {
            kind: err_kind,
            cause: self.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        assert_eq!(AppErrorKind::RequestParamInvalid.message(), "请求参数无效");
        assert_eq!(AppErrorKind::MissingCredential.message(), "缺少访问凭证");
        assert_eq!(AppErrorKind::ResourceNotFound.message(), "请求资源不存在");
    }

    #[test]
    fn test_http_codes() {
        assert_eq!(
            AppErrorKind::RequestParamInvalid.http_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppErrorKind::MissingCredential.http_code(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            AppErrorKind::ResourceNotFound.http_code(),
            StatusCode::NOT_FOUND
        );
    }

    #[test]
    fn test_app_codes() {
        assert_eq!(AppErrorKind::RequestParamInvalid.app_code(), -40000);
        assert_eq!(AppErrorKind::MissingCredential.app_code(), -40100);
        assert_eq!(AppErrorKind::ResourceNotFound.app_code(), -40400);
    }

    #[test]
    fn test_with_context() {
        let error = AppError::new("测试错误", AppErrorKind::InternalError);
        assert_eq!(error.message(), "服务端内部错误");

        let error_with_context = error.context("详细信息");
        assert_eq!(
            format!("{}", error_with_context),
            "服务端内部错误:\n详细信息"
        );
        assert_eq!(
            format!("{:?}", error_with_context),
            "服务端内部错误:\n详细信息\n\nCaused by:\n    测试错误"
        );

        let error_with_context = error_with_context.with_context(|| "更多详细信息");
        assert_eq!(
            format!("{}", error_with_context),
            "服务端内部错误:\n更多详细信息"
        );
        assert_eq!(
            format!("{:?}", error_with_context),
            "服务端内部错误:\n更多详细信息\n\nCaused by:\n    0: 详细信息\n    1: 测试错误"
        );
    }

    #[test]
    fn test_into_app_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "文件未找到");
        let app_error: AppError = io_error.with_err_kind(AppErrorKind::ResourceNotFound);

        assert_eq!(app_error.message(), "请求资源不存在");
        assert_eq!(format!("{}", app_error), "请求资源不存在:\n文件未找到");
    }
}
