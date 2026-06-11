use crate::blogs::models::LatestArticles;
use crate::blogs::service;
use crate::common::{ApiResponse, AppResult, AppState};
use axum::extract::State;

#[utoipa::path(
    get,
    path = "/demo",
    tag = "blogs",
    responses((status = 200, body = ApiResponse<Vec<LatestArticles>>))
)]
pub async fn latest_articles(State(app_state): State<AppState>) -> AppResult<Vec<LatestArticles>> {
    service::latest_articles(&app_state.db).await
}
