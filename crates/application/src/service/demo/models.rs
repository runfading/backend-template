use domain::biz::demo::models::{CreateDemo, Demo, UpdateDemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateDemoCommand {
    pub name: String,
    pub description: Option<String>,
}

impl From<CreateDemoCommand> for CreateDemo {
    fn from(value: CreateDemoCommand) -> Self {
        Self {
            name: value.name,
            description: value.description,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateDemoCommand {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl From<UpdateDemoCommand> for UpdateDemo {
    fn from(value: UpdateDemoCommand) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
        }
    }
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

impl From<Demo> for DemoDetails {
    fn from(value: Demo) -> Self {
        Self {
            id: value.id.value(),
            name: value.name,
            description: value.description,
        }
    }
}

impl From<DemoDetails> for Demo {
    fn from(value: DemoDetails) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
        }
    }
}
