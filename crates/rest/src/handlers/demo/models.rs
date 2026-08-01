use application::service::demo::models::{CreateDemoCommand, DemoDetails};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct CreateDemoReqDTO {
    pub name: String,
    pub description: Option<String>,
}

impl From<CreateDemoReqDTO> for CreateDemoCommand {
    fn from(value: CreateDemoReqDTO) -> Self {
        Self {
            name: value.name,
            description: value.description,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct UpdateDemoReqDTO {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct PatchDemoReqDTO {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct DemoDTO {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

impl From<DemoDetails> for DemoDTO {
    fn from(value: DemoDetails) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}
