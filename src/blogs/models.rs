use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Debug, ToSchema)]
pub struct LatestArticles {
    pub name: String,
    pub id: i64,
}

#[derive(sqlx::FromRow, Serialize, Debug, PartialEq, Eq, ToSchema)]
pub struct Article {
    pub id: i64,
    pub folder_id: i64,
    pub title: String,
    pub description: String,
    pub cover_url: Option<String>,
    pub author: String,
    pub content: String,
    pub is_pinned: bool,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateArticle {
    pub folder_id: i64,
    pub title: String,
    pub description: String,
    pub cover_url: Option<String>,
    pub author: String,
    pub content: String,
    pub is_pinned: Option<bool>,
    pub tag_ids: Option<Vec<i64>>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdateArticle {
    pub folder_id: i64,
    pub title: String,
    pub description: String,
    pub cover_url: Option<String>,
    pub author: String,
    pub content: String,
    pub is_pinned: Option<bool>,
    pub tag_ids: Option<Vec<i64>>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpsertArticle {
    pub id: Option<i64>,
    pub folder_id: i64,
    pub title: String,
    pub description: String,
    pub cover_url: Option<String>,
    pub author: String,
    pub content: String,
    pub is_pinned: Option<bool>,
    pub tag_ids: Option<Vec<i64>>,
}

#[derive(Deserialize, Debug, IntoParams)]
pub struct ArticleQuery {
    pub folder_id: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
