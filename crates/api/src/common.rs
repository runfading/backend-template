use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::fmt::Debug;
use tracing::{debug, error};
use utoipa::ToSchema;

pub const SUCCESS_CODE: u32 = 0;
pub const SUCCESS_MESSAGE: &str = "success";
pub const NOT_FOUND_CODE: u32 = 4040;
pub const INTERNAL_SERVER_ERROR_CODE: u32 = 9999;

#[derive(Debug, Serialize, ToSchema)]
pub struct Empty {}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: SUCCESS_CODE,
            message: SUCCESS_MESSAGE.to_string(),
            data: Some(data),
        }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> ApiResponse<U> {
        ApiResponse {
            code: self.code,
            message: self.message,
            data: self.data.map(f),
        }
    }
}

impl ApiResponse<Empty> {
    pub fn err(code: u32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    pub fn empty_ok() -> Self {
        Self {
            code: SUCCESS_CODE,
            message: SUCCESS_MESSAGE.to_string(),
            data: Some(Empty {}),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize + Debug,
{
    fn into_response(self) -> Response {
        if tracing::enabled!(tracing::Level::DEBUG) {
            if let Ok(json) = serde_json::to_string(&self) {
                debug!("response: {}", json);
            } else {
                error!("failed to serialize response");
            }
        }

        Json(self).into_response()
    }
}
