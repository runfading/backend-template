use crate::blogs::models::{
    Article, ArticleQuery, CreateArticle, LatestArticles, UpdateArticle, UpsertArticle,
};
use crate::blogs::service;
use crate::common::{ApiResponse, AppResult, AppState};
use axum::extract::{Path, Query, State};
use axum::Json;

#[utoipa::path(
    get,
    path = "/latest_articles",
    tag = "blogs",
    responses((status = 200, body = ApiResponse<Vec<LatestArticles>>))
)]
pub async fn latest_articles(State(app_state): State<AppState>) -> AppResult<Vec<LatestArticles>> {
    service::latest_articles(&app_state.db).await
}

#[utoipa::path(
    get,
    path = "/recent_articles",
    tag = "blogs",
    params(ArticleQuery),
    responses((status = 200, body = ApiResponse<Vec<Article>>))
)]
pub async fn list_articles(
    State(app_state): State<AppState>,
    Query(query): Query<ArticleQuery>,
) -> AppResult<Vec<Article>> {
    service::list_articles(&app_state.db, query).await
}

#[utoipa::path(
    get,
    path = "/articles/{id}",
    tag = "blogs",
    params(("id" = i64, Path, description = "Article id")),
    responses((status = 200, body = ApiResponse<Article>))
)]
pub async fn get_article(
    State(app_state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Article> {
    service::get_article(&app_state.db, id).await
}

#[utoipa::path(
    post,
    path = "/articles",
    tag = "blogs",
    request_body = CreateArticle,
    responses((status = 200, body = ApiResponse<i64>))
)]
pub async fn create_article(
    State(app_state): State<AppState>,
    Json(article): Json<CreateArticle>,
) -> AppResult<i64> {
    service::create_article(&app_state.db, article).await
}

#[utoipa::path(
    put,
    path = "/articles/{id}",
    tag = "blogs",
    params(("id" = i64, Path, description = "Article id")),
    request_body = UpdateArticle,
    responses((status = 200, body = ApiResponse<i64>))
)]
pub async fn update_article(
    State(app_state): State<AppState>,
    Path(id): Path<i64>,
    Json(article): Json<UpdateArticle>,
) -> AppResult<i64> {
    service::update_article(&app_state.db, id, article).await
}

#[utoipa::path(
    delete,
    path = "/articles/{id}",
    tag = "blogs",
    params(("id" = i64, Path, description = "Article id")),
    responses((status = 200, description = "success"))
)]
pub async fn delete_article(
    State(app_state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<()> {
    service::delete_article(&app_state.db, id).await
}

#[utoipa::path(
    put,
    path = "/upsert",
    tag = "blogs",
    request_body = UpsertArticle,
    responses((status = 200, body = ApiResponse<i64>))
)]
pub async fn upsert(
    State(app_state): State<AppState>,
    Json(upsert): Json<UpsertArticle>,
) -> AppResult<i64> {
    service::upsert_articles(&app_state.db, upsert).await
}
