use crate::common::{ApiResponse, ApiResult, Empty};
use crate::handlers::demo::DemoState;
use crate::handlers::demo::models::{CreateDemoReqDTO, DemoDTO, UpdateDemoReqDTO};
use axum::extract::{Path, State};

#[utoipa::path(
    get,
    path = "/list",
    tag = "demo",
    responses((status = 200, body = ApiResponse<Vec<DemoDTO>>))
)]
pub async fn list(State(app_state): State<DemoState>) -> ApiResult<Vec<DemoDTO>> {
    ApiResponse::vec(app_state.demo_service.list().await?)
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "demo",
    params(("id" = i64, Path, description = "note id")),
    responses((status = 200, body = ApiResponse<DemoDTO>))
)]
pub async fn get(State(app_state): State<DemoState>, Path(id): Path<i64>) -> ApiResult<DemoDTO> {
    ApiResponse::ok(app_state.demo_service.get(id).await?)
}

#[utoipa::path(
    post,
    path = "/create",
    tag = "demo",
    request_body = CreateDemoReqDTO,
    responses((status = 200, body = ApiResponse<DemoDTO>))
)]
pub async fn create(
    State(app_state): State<DemoState>,
    axum::Json(input): axum::Json<CreateDemoReqDTO>,
) -> ApiResult<DemoDTO> {
    ApiResponse::ok(app_state.demo_service.create(input.into()).await?)
}

#[utoipa::path(
    put,
    path = "/update",
    tag = "demo",
    request_body = UpdateDemoReqDTO,
    responses((status = 200, body = ApiResponse<DemoDTO>))
)]
pub async fn update(
    State(app_state): State<DemoState>,
    axum::Json(input): axum::Json<UpdateDemoReqDTO>,
) -> ApiResult<DemoDTO> {
    ApiResponse::ok(app_state.demo_service.update(input.into()).await?)
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "demo",
    params(("id" = i64, Path, description = "note id")),
    responses((status = 200, body = ApiResponse<Empty>))
)]
pub async fn remove(State(app_state): State<DemoState>, Path(id): Path<i64>) -> ApiResult<Empty> {
    app_state.demo_service.delete(id).await?;
    ApiResponse::empty_ok()
}
