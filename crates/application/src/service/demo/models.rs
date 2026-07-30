use crate::service::demo::entity::Model;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateDemoCommand {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateDemoCommand {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl UpdateDemoCommand {
    pub fn new(id: i64, name: Option<String>, description: Option<String>) -> Self {
        UpdateDemoCommand {
            id,
            name,
            description,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoDetails {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

impl From<Model> for DemoDetails {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}
