use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

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
