use crate::AppState;
use crate::error::ApiResult;
use crate::handlers::demo::models::{
    CreateDemoReqDTO, DemoDTO, PatchDemoReqDTO, UpdateDemoReqDTO,
};
use api::{ApiResponse, Empty, PageQuery, PageResult};
use application::service::demo as demo_application;
use application::service::demo::models::{PatchDemoCommand, UpdateDemoCommand};
use axum::extract::{Path, Query, State};

#[utoipa::path(
    get,
    path = "/list",
    tag = "demo",
    responses((status = 200, body = ApiResponse<Vec<DemoDTO>>))
)]
pub async fn list(State(state): State<AppState>) -> ApiResult<Vec<DemoDTO>> {
    Ok(demo_application::list(state.db())
        .await?
        .map(|items| items.into_iter().map(Into::into).collect()))
}

#[utoipa::path(
    get,
    path = "/page",
    tag = "demo",
    params(PageQuery),
    responses((status = 200, body = ApiResponse<PageResult<DemoDTO>>))
)]
pub async fn page(
    State(state): State<AppState>,
    Query(query): Query<PageQuery>,
) -> ApiResult<PageResult<DemoDTO>> {
    Ok(demo_application::page(state.db(), query)
        .await?
        .map(|page| page.map(Into::into)))
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "demo",
    params(("id" = i64, Path, description = "demo id")),
    responses((status = 200, body = ApiResponse<DemoDTO>))
)]
pub async fn get(State(state): State<AppState>, Path(id): Path<i64>) -> ApiResult<DemoDTO> {
    Ok(demo_application::get(state.db(), id).await?.map(Into::into))
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
    Ok(demo_application::create(state.db(), input.into())
        .await?
        .map(Into::into))
}

#[utoipa::path(
    put,
    path = "/{id}",
    tag = "demo",
    params(("id" = i64, Path, description = "demo id")),
    request_body = UpdateDemoReqDTO,
    responses((status = 200, body = ApiResponse<DemoDTO>))
)]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    axum::Json(input): axum::Json<UpdateDemoReqDTO>,
) -> ApiResult<DemoDTO> {
    let command = UpdateDemoCommand::new(id, input.name, input.description);
    Ok(demo_application::update(state.db(), command)
        .await?
        .map(Into::into))
}

#[utoipa::path(
    patch,
    path = "/{id}",
    tag = "demo",
    params(("id" = i64, Path, description = "demo id")),
    request_body = PatchDemoReqDTO,
    responses((status = 200, body = ApiResponse<DemoDTO>))
)]
pub async fn patch(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    axum::Json(input): axum::Json<PatchDemoReqDTO>,
) -> ApiResult<DemoDTO> {
    let command = PatchDemoCommand::new(id, input.name, input.description);
    Ok(demo_application::patch(state.db(), command)
        .await?
        .map(Into::into))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "demo",
    params(("id" = i64, Path, description = "demo id")),
    responses((status = 200, body = ApiResponse<Empty>))
)]
pub async fn remove(State(state): State<AppState>, Path(id): Path<i64>) -> ApiResult<Empty> {
    Ok(demo_application::delete(state.db(), id).await?)
}
