use crate::AppState;
use crate::common::{ApiResponse, ApiResult, Empty};
use crate::handlers::demo::models::{CreateDemoReqDTO, DemoDTO, UpdateDemoReqDTO};
use application::service::demo as demo_application;
use axum::extract::{Path, State};

#[utoipa::path(
    get,
    path = "/list",
    tag = "demo",
    responses((status = 200, body = ApiResponse<Vec<DemoDTO>>))
)]
pub async fn list(State(state): State<AppState>) -> ApiResult<Vec<DemoDTO>> {
    ApiResponse::vec(demo_application::list(state.db()).await?)
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "demo",
    params(("id" = i64, Path, description = "note id")),
    responses((status = 200, body = ApiResponse<DemoDTO>))
)]
pub async fn get(State(state): State<AppState>, Path(id): Path<i64>) -> ApiResult<DemoDTO> {
    ApiResponse::ok(demo_application::get(state.db(), id).await?)
}

#[utoipa::path(
    post,
    path = "/create",
    tag = "demo",
    request_body = CreateDemoReqDTO,
    responses((status = 200, body = ApiResponse<DemoDTO>))
)]
pub async fn create(
    State(state): State<AppState>,
    axum::Json(input): axum::Json<CreateDemoReqDTO>,
) -> ApiResult<DemoDTO> {
    ApiResponse::ok(demo_application::create(state.db(), input.into()).await?)
}

#[utoipa::path(
    put,
    path = "/update",
    tag = "demo",
    request_body = UpdateDemoReqDTO,
    responses((status = 200, body = ApiResponse<DemoDTO>))
)]
pub async fn update(
    State(state): State<AppState>,
    axum::Json(input): axum::Json<UpdateDemoReqDTO>,
) -> ApiResult<DemoDTO> {
    ApiResponse::ok(demo_application::update(state.db(), input.into()).await?)
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "demo",
    params(("id" = i64, Path, description = "note id")),
    responses((status = 200, body = ApiResponse<Empty>))
)]
pub async fn remove(State(state): State<AppState>, Path(id): Path<i64>) -> ApiResult<Empty> {
    demo_application::delete(state.db(), id).await?;
    ApiResponse::empty_ok()
}
