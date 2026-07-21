use crate::common::ApplicationResult;
use crate::service::demo::models::{CreateDemoCommand, DemoDetails, UpdateDemoCommand};

#[async_trait::async_trait]
pub trait DemoApplicationService: Send + Sync {
    async fn list(&self) -> ApplicationResult<Vec<DemoDetails>>;

    async fn get(&self, id: i64) -> ApplicationResult<DemoDetails>;

    async fn create(&self, command: CreateDemoCommand) -> ApplicationResult<DemoDetails>;

    async fn update(&self, command: UpdateDemoCommand) -> ApplicationResult<DemoDetails>;

    async fn delete(&self, id: i64) -> ApplicationResult<()>;
}
