use crate::error::AppError;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use sqlx::PgPool;
use std::fmt::Debug;
use tracing::{debug, error};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;

pub struct RouteRegistrar {
    pub routes_fn: fn() -> OpenApiRouter<AppState>,
}

inventory::collect!(RouteRegistrar);

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
}

pub type DbPool = PgPool;

pub type AppResult<T> = Result<ApiResponse<T>, AppError>;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "success".into(),
            data: Some(data),
        }
    }
}

impl ApiResponse<()> {
    pub fn err(code: u32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
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
