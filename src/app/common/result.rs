// ********************* import ********************* //
use std::fmt::Display;

use anyhow::anyhow;

use super::error::{AppError, AppErrorKind, IntoAppError};

// ********************* content ********************* //
pub type AppResult<T> = Result<T, AppError>;

pub trait IntoAppResult<T> {
    fn with_err_kind(self, err_kind: AppErrorKind) -> AppResult<T>;
}

impl<T, E> IntoAppResult<T> for Result<T, E>
where
    E: IntoAppError,
{
    fn with_err_kind(self, err_kind: AppErrorKind) -> AppResult<T> {
        self.map_err(|e| e.with_err_kind(err_kind))
    }
}

impl<T> IntoAppResult<T> for Option<T> {
    fn with_err_kind(self, err_kind: AppErrorKind) -> AppResult<T> {
        self.ok_or_else(|| {
            AppError::new(
                anyhow!("Unexpected None: Expected a value but found None."),
                err_kind,
            )
        })
    }
}

pub trait ContextExt<T> {
    fn context<C>(self, context: C) -> AppResult<T>
    where
        C: Display + Send + Sync + 'static;
    fn with_context<C, F>(self, context: F) -> AppResult<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T> ContextExt<T> for AppResult<T> {
    fn context<C>(self, context: C) -> AppResult<T>
    where
        C: Display + Send + Sync + 'static,
    {
        self.map_err(|e| e.context(context))
    }

    fn with_context<C, F>(self, context: F) -> AppResult<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|e| e.context(context()))
    }
}

pub trait WrapToAppResult<T> {
    fn wrap<C>(self, context: C, err_kind: AppErrorKind) -> AppResult<T>
    where
        C: Display + Send + Sync + 'static;
    fn wrap_with<C, F>(self, context: F, err_kind: AppErrorKind) -> AppResult<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, R> WrapToAppResult<T> for R
where
    R: IntoAppResult<T>,
{
    fn wrap<C>(self, context: C, err_kind: AppErrorKind) -> AppResult<T>
    where
        C: Display + Send + Sync + 'static,
    {
        self.with_err_kind(err_kind).context(context)
    }
    fn wrap_with<C, F>(self, context: F, err_kind: AppErrorKind) -> AppResult<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.with_err_kind(err_kind).with_context(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_into_app_result() {
        let result = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "文件未找到",
        ));
        let app_result: AppResult<()> = result.with_err_kind(AppErrorKind::InternalError);
        assert!(app_result.is_err());
        assert_eq!(app_result.err().unwrap().message(), "服务端内部错误");
    }

    #[test]
    fn test_option_into_app_result() {
        let option: Option<()> = None;
        let app_result: AppResult<()> = option.with_err_kind(AppErrorKind::ResourceNotFound);
        assert!(app_result.is_err());
        assert_eq!(app_result.err().unwrap().message(), "请求资源不存在");
    }

    #[test]
    fn test_context() {
        let result: AppResult<()> = Err(AppError::new("测试错误", AppErrorKind::InternalError));
        let context_result = result.context("详细信息");
        assert!(context_result.is_err());
        assert_eq!(
            format!("{:?}", context_result),
            "Err(服务端内部错误:\n详细信息\n\nCaused by:\n    测试错误)"
        );
    }

    #[test]
    fn test_result_wrap_to_app_result() {
        let result = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "文件未找到",
        ));
        let app_result: AppResult<()> = result.wrap("详细信息", AppErrorKind::ResourceNotFound);
        assert!(app_result.is_err());
        assert_eq!(
            format!("{:?}", app_result),
            "Err(请求资源不存在:\n详细信息\n\nCaused by:\n    文件未找到)"
        );
    }

    #[test]
    fn test_option_wrap_to_app_result() {
        let option: Option<()> = None;
        let app_result: AppResult<()> = option.wrap("详细信息", AppErrorKind::ResourceNotFound);
        assert!(app_result.is_err());
        assert_eq!(
            format!("{:?}", app_result),
            "Err(请求资源不存在:\n详细信息\n\nCaused by:\n    Unexpected None: Expected a value but found None.)"
        );
    }
}
