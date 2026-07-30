use crate::error::ApiError;
use application::common::ApplicationResponse;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::fmt::Debug;
use tracing::{debug, error};
use utoipa::ToSchema;

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

#[derive(Debug, Serialize, ToSchema)]
pub struct Empty {}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok<R>(data: ApplicationResponse<T>) -> ApiResult<R>
    where
        R: From<T>,
    {
        Ok(ApiResponse {
            code: data.code,
            message: data.message,
            data: data.data.map(R::from),
        })
    }

    pub fn vec<R>(data: ApplicationResponse<Vec<T>>) -> ApiResult<Vec<R>>
    where
        R: From<T>,
    {
        Ok(ApiResponse {
            code: data.code,
            message: data.message,
            data: data.data.map(|vec| vec.into_iter().map(R::from).collect()),
        })
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

    pub fn empty_ok() -> ApiResult<Empty> {
        Ok(Self {
            code: 0,
            message: "success".to_string(),
            data: Some(Empty {}),
        })
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

impl<T, R> From<ApplicationResponse<T>> for ApiResponse<R>
where
    R: From<T>,
{
    fn from(value: ApplicationResponse<T>) -> Self {
        Self {
            code: value.code,
            message: value.message,
            data: value.data.map(R::from),
        }
    }
}
