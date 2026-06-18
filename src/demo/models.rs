use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct SimpleDemo {
    pub name: String,
    pub id: i64,
}

impl Into<SimpleDemo> for Model {
    fn into(self) -> SimpleDemo {
        SimpleDemo {
            name: self.name,
            id: self.id,
        }
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateDemo {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdateDemo {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "demo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub created_at: TimeDateTimeWithTimeZone,
}

impl ActiveModelBehavior for ActiveModel {}
