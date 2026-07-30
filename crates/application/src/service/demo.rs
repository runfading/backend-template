mod entity;
pub mod models;

use crate::common::{ApplicationResponse, ApplicationResult};
use crate::error::ApplicationError;
use crate::service::demo::entity::{ActiveModel, Entity};
use crate::service::demo::models::{CreateDemoCommand, DemoDetails, UpdateDemoCommand};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TryIntoModel};

pub async fn list(db: &DatabaseConnection) -> ApplicationResult<Vec<DemoDetails>> {
    let demos = Entity::find().all(db).await?;
    ApplicationResponse::vec(demos)
}

pub async fn get(db: &DatabaseConnection, id: i64) -> ApplicationResult<DemoDetails> {
    let demo = Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| ApplicationError::NotFound(format!("Demo id: {id}")))?;
    ApplicationResponse::ok(demo)
}

pub async fn create(
    db: &DatabaseConnection,
    command: CreateDemoCommand,
) -> ApplicationResult<DemoDetails> {
    let active_model = ActiveModel {
        name: Set(command.name),
        description: Set(command.description),
        created_at: Set(time::OffsetDateTime::now_utc()),
        ..Default::default()
    };

    let demo = active_model.save(db).await?.try_into_model()?;
    ApplicationResponse::ok(demo)
}

pub async fn update(
    db: &DatabaseConnection,
    command: UpdateDemoCommand,
) -> ApplicationResult<DemoDetails> {
    let model = Entity::find_by_id(command.id)
        .one(db)
        .await?
        .ok_or_else(|| ApplicationError::NotFound(format!("Demo id: {}", command.id)))?;
    let mut active_model: ActiveModel = model.into();

    if let Some(name) = command.name {
        active_model.name = Set(name);
    }
    active_model.description = Set(command.description);

    let demo = active_model.update(db).await?.try_into_model()?;
    ApplicationResponse::ok(demo)
}

pub async fn delete(db: &DatabaseConnection, id: i64) -> ApplicationResult<()> {
    let result = Entity::delete_by_id(id).exec(db).await?;
    if result.rows_affected == 0 {
        return Err(ApplicationError::NotFound(format!("Demo id: {id}")));
    }
    ApplicationResponse::empty_ok()
}
