use crate::common::ApiResponse;
use application::error::ApplicationError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    ApplicationError(#[from] ApplicationError),
}

pub type ApiResult<T> = Result<T, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            ApiError::ApplicationError(error) => match error {
                ApplicationError::BizError(biz) => {
                    (StatusCode::OK, biz.code, biz.message.to_string())
                }
                ApplicationError::NotFound(message) => (StatusCode::NOT_FOUND, 4040, message),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    9999,
                    "Internal Server Error".to_string(),
                ),
            },
        };

        if status.is_server_error() {
            error!(status = %status, code, message, "request failed");
        }

        (status, ApiResponse::err(code, message.to_string())).into_response()
    }
}
