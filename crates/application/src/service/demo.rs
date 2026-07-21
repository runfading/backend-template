pub mod models;
pub mod service;

use crate::common::{ApplicationResponse, ApplicationResult};
use crate::service::demo::models::{CreateDemoCommand, DemoDetails, UpdateDemoCommand};
use crate::service::demo::service::DemoApplicationService;
use domain::biz::demo::domain::DemoRepository;

#[derive(Debug, Clone)]
pub struct DemoService<R> {
    repository: R,
}

impl<R> DemoService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl<R> DemoApplicationService for DemoService<R>
where
    R: DemoRepository + Send + Sync,
{
    async fn list(&self) -> ApplicationResult<Vec<DemoDetails>> {
        // self.repository.list().await?
        ApplicationResponse::vec(self.repository.list().await?)
    }

    async fn get(&self, id: i64) -> ApplicationResult<DemoDetails> {
        ApplicationResponse::optional(self.repository.find_by_id(id).await?)
    }

    async fn create(&self, command: CreateDemoCommand) -> ApplicationResult<DemoDetails> {
        ApplicationResponse::ok(self.repository.save(command.into()).await?)
    }

    async fn update(&self, command: UpdateDemoCommand) -> ApplicationResult<DemoDetails> {
        ApplicationResponse::ok(self.repository.update(command.into()).await?)
    }

    async fn delete(&self, id: i64) -> ApplicationResult<()> {
        self.repository.delete(id).await?;
        ApplicationResponse::empty_ok()
    }
}
