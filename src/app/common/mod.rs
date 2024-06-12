pub mod error;
pub mod response;
pub mod result;

pub mod prelude {
    pub use super::error::{AppError, AppErrorKind, IntoAppError};
    pub use super::response::AppResponse;
    pub use super::result::{AppResult, ContextExt, IntoAppResult, WrapToAppResult};
}
