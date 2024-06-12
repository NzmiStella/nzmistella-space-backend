// ********************* mod ********************* //
pub mod user;

pub mod prelude {
    pub use super::user::{admin_router as user_admin_router, public_router as user_public_router};
}

// ********************* import ********************* //
use axum::{async_trait, extract::FromRequestParts, http::header};
use http::request::Parts;

use crate::app::common::prelude::*;

// ********************* content ********************* //
pub trait HandlerAsyncSafe = Send + Sync + 'static;

struct BearerToken(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for BearerToken {
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());

        match auth_header {
            Some(header) if header.starts_with("Bearer ") => {
                let token = header.trim_start_matches("Bearer ").to_string();
                Ok(BearerToken(token))
            }
            Some(_) => Err(AppError::new(
                "Authorization header must start with 'Bearer'",
                AppErrorKind::MalformedCredential,
            )),
            None => Err(AppError::new(
                "Authorization header not found",
                AppErrorKind::MissingCredential,
            )),
        }
    }
}
