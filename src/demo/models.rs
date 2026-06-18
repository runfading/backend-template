use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct SimpleDemo {
    pub name: String,
    pub id: i64,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateDemo {
    pub folder_id: i64,
    pub title: String,
    pub description: String,
    pub cover_url: Option<String>,
    pub author: String,
    pub content: String,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdateDemo {
    pub id: i64,
    pub folder_id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub author: Option<String>,
    pub content: Option<String>,
    pub is_pinned: Option<bool>,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "demo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub folder_id: i64,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub cover_url: Option<String>,
    pub author: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub is_pinned: bool,
    pub created_at: DateTimeWithTimeZone,
}

impl ActiveModelBehavior for ActiveModel {}
