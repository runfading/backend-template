use crate::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateDemo {
    pub name: String,
    pub description: Option<String>,
}

impl CreateDemo {
    pub fn new(name: String, description: Option<String>) -> Self {
        CreateDemo { name, description }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateDemo {
    pub id: DemoId,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl UpdateDemo {
    pub fn new(id: i64, name: Option<String>, description: Option<String>) -> Self {
        UpdateDemo {
            id: id.into(),
            name,
            description,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Demo {
    pub id: DemoId,
    pub name: String,
    pub description: Option<String>,
}

impl Demo {
    pub fn restore(
        id: i64,
        name: String,
        description: Option<String>,
    ) -> Result<Demo, DomainError> {
        Ok(Self {
            id: id.into(),
            name,
            description,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DemoId(pub i64);

impl DemoId {
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl From<i64> for DemoId {
    fn from(value: i64) -> Self {
        DemoId(value)
    }
}
