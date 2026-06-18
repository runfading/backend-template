use crate::common::{ApiResponse, DbError};
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
    Db(#[from] DbError),

    #[error("")]
    BizError(#[from] BizError),

    #[error("{0}")]
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 先记录完整错误
        if !matches!(self, AppError::NotFound(_) | AppError::BizError(_)) {
            error!(error = ?self, "internal server error");
        }

        let (status, code, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, 0, msg),
            AppError::BizError(BizError { code, message }) => {
                (StatusCode::OK, code, message.to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                9999,
                "Internal Server Error".to_string(),
            ),
        };

        (status, ApiResponse::err(code, message.to_string())).into_response()
    }
}
