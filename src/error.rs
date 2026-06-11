use crate::common::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct BizError {
    pub code: u32,
    pub message: &'static str,
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("database error")]
    Db(#[from] sqlx::Error),

    #[error("")]
    BizError(#[from] BizError),

    #[error("not found")]
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 先记录完整错误
        if !matches!(self, AppError::NotFound | AppError::BizError(_)) {
            error!(error = ?self, "internal server error");
        }

        let (status, code, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, 0, "404 not found"),
            AppError::BizError(BizError { code, message }) => (StatusCode::OK, code, message),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                9999,
                "Internal Server Error",
            ),
        };

        (status, ApiResponse::err(code, message.to_string())).into_response()
    }
}
