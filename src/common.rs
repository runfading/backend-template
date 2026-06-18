use crate::error::AppError;
use axum::response::{IntoResponse, Response};
use axum::Json;
use sea_orm::DatabaseConnection;
use serde::Serialize;
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

pub type DbPool = DatabaseConnection;
pub type AppResult<T> = Result<ApiResponse<T>, AppError>;
pub type DbError = sea_orm::DbErr;

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
            code: 0,
            message: "success".into(),
            data: Some(data),
        }
    }
}

impl ApiResponse<Empty> {
    pub fn err(code: u32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }

    pub fn empty_ok() -> Self {
        Self::ok(Empty {})
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
