// ********************* import ********************* //
use axum::response::{IntoResponse, Json, Response};
use http::StatusCode;
use serde::Serialize;
use serde_json::{json, Value};

use super::{error::AppError, result::AppResult};

// ********************* content ********************* //
pub struct AppResponse {
    http_code: StatusCode,
    body: Value,
}

impl AppResponse {
    pub fn err(message: &str, app_code: i32, http_code: StatusCode) -> Self {
        let body = json!({
            "code": app_code,
            "message": message,
            "data": null,
        });
        Self { http_code, body }
    }

    pub fn succ<T: Serialize>(data: T) -> Self {
        let body = json!({
            "code": 0,
            "message": "成功",
            "data": data,
        });
        Self {
            http_code: StatusCode::OK,
            body,
        }
    }
}

impl From<AppError> for AppResponse {
    fn from(error: AppError) -> Self {
        Self::err(&format!("{}", error), error.app_code(), error.http_code())
    }
}

impl<T: Serialize> From<AppResult<T>> for AppResponse {
    fn from(result: AppResult<T>) -> Self {
        match result {
            Ok(v) => AppResponse::succ(v),
            Err(e) => e.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        AppResponse::from(self).into_response()
    }
}

impl IntoResponse for AppResponse {
    fn into_response(self) -> Response {
        (self.http_code, Json(self.body)).into_response()
    }
}

impl<T> std::ops::FromResidual<AppResult<T>> for AppResponse {
    fn from_residual(residual: AppResult<T>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => e.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::error::{AppError, AppErrorKind};
    use super::*;
    use axum::body::to_bytes;
    use serde_json::Value;
    use tokio::runtime::Runtime;

    #[test]
    fn test_response_conversion() {
        async fn extract_json_body(response: Response) -> Value {
            let body = to_bytes(response.into_body(), 99999).await.unwrap();
            serde_json::from_slice(&body).unwrap()
        }
        let rt = Runtime::new().unwrap();

        // 测试成功的响应
        let ok_response: AppResponse = Ok(json!({"key": "value"})).into();
        let axum_response = ok_response.into_response();
        let ok_json = rt.block_on(extract_json_body(axum_response));
        assert_eq!(ok_json.get("code").unwrap(), &json!(0));
        assert_eq!(ok_json.get("data").unwrap(), &json!({"key": "value"}));

        // 测试错误的响应
        let err_response: AppResponse =
            Result::<(), _>::Err(AppError::new("Failed", AppErrorKind::InternalError)).into();
        let axum_response = err_response.into_response();
        let err_json = rt.block_on(extract_json_body(axum_response));
        assert_eq!(err_json.get("code").unwrap(), &json!(-50000));
        assert_eq!(
            err_json.get("message").unwrap(),
            &json!("服务端内部错误:\nFailed")
        );
    }
}
