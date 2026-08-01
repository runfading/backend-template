use api::{ApiResponse, INTERNAL_SERVER_ERROR_CODE, NOT_FOUND_CODE};
use application::error::ApplicationError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    ApplicationError(#[from] ApplicationError),
}

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            ApiError::ApplicationError(ApplicationError::BizError(biz)) => {
                (StatusCode::OK, biz.code, biz.message.to_string())
            }
            ApiError::ApplicationError(ApplicationError::NotFound(message)) => {
                (StatusCode::NOT_FOUND, NOT_FOUND_CODE, message.clone())
            }
            other => {
                error!(error = %other, "internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    INTERNAL_SERVER_ERROR_CODE,
                    "Internal Server Error".to_string(),
                )
            }
        };

        (status, ApiResponse::err(code, message)).into_response()
    }
}
