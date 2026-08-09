use api::{ApiErrorCode, ApiResponse};
use application::error::ApplicationError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    ApplicationError(#[from] ApplicationError),

    #[error("unauthorized: {0}")]
    Unauthorized(&'static str),

    #[error("failed to create access token")]
    TokenCreation(#[source] jsonwebtoken::errors::Error),
}

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            ApiError::ApplicationError(ApplicationError::BizError(biz)) => {
                (StatusCode::OK, biz.code, biz.message.to_string())
            }
            ApiError::ApplicationError(ApplicationError::NotFound(message)) => (
                StatusCode::NOT_FOUND,
                ApiErrorCode::NOT_FOUND.code(),
                message.clone(),
            ),
            ApiError::Unauthorized(message) => (
                StatusCode::UNAUTHORIZED,
                ApiErrorCode::UNAUTHORIZED.code(),
                message.to_string(),
            ),
            other => {
                error!(error = %other, "internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorCode::INTERNAL_SERVER.code(),
                    ApiErrorCode::INTERNAL_SERVER.message().to_string(),
                )
            }
        };

        (status, ApiResponse::err(code, message)).into_response()
    }
}
