use crate::common::{ApiResponse, AppResult, AppState, Empty};
use crate::demo::models::{CreateDemo, SimpleDemo, UpdateDemo};
use crate::demo::service;
use axum::extract::{Path, State};

#[utoipa::path(
    get,
    path = "/list",
    tag = "demo",
    responses((status = 200, body = ApiResponse<Vec<SimpleDemo>>))
)]
pub async fn list(State(app_state): State<AppState>) -> AppResult<Vec<SimpleDemo>> {
    service::list(&app_state.db).await
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "demo",
    params(("id" = i64, Path, description = "note id")),
    responses((status = 200, body = ApiResponse<SimpleDemo>))
)]
pub async fn get(State(app_state): State<AppState>, Path(id): Path<i64>) -> AppResult<SimpleDemo> {
    service::get(&app_state.db, id).await
}

#[utoipa::path(
    post,
    path = "/create",
    tag = "demo",
    request_body = CreateDemo,
    responses((status = 200, body = ApiResponse<SimpleDemo>))
)]
pub async fn create(
    State(app_state): State<AppState>,
    axum::Json(input): axum::Json<CreateDemo>,
) -> AppResult<SimpleDemo> {
    service::create(&app_state.db, input).await
}

#[utoipa::path(
    put,
    path = "/update",
    tag = "demo",
    request_body = UpdateDemo,
    responses((status = 200, body = ApiResponse<SimpleDemo>))
)]
pub async fn update(
    State(app_state): State<AppState>,
    axum::Json(input): axum::Json<UpdateDemo>,
) -> AppResult<SimpleDemo> {
    service::update(&app_state.db, input).await
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "demo",
    params(("id" = i64, Path, description = "note id")),
    responses((status = 200, body = ApiResponse<Empty>))
)]
pub async fn remove(State(app_state): State<AppState>, Path(id): Path<i64>) -> AppResult<Empty> {
    service::delete(&app_state.db, id).await
}
