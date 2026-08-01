mod entity;
pub mod models;

use crate::common::ApplicationResult;
use crate::error::ApplicationError;
use crate::service::demo::entity::{ActiveModel, Entity};
use crate::service::demo::models::{
    CreateDemoCommand, DemoDetails, PatchDemoCommand, UpdateDemoCommand,
};
use api::{ApiResponse, Empty, PageQuery, PageResult};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TryIntoModel};

pub async fn list(db: &DatabaseConnection) -> ApplicationResult<Vec<DemoDetails>> {
    let demos: Vec<DemoDetails> = Entity::find()
        .all(db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(ApiResponse::ok(demos))
}

pub async fn page(
    db: &DatabaseConnection,
    query: PageQuery,
) -> ApplicationResult<PageResult<DemoDetails>> {
    let query = query.normalized();
    let paginator = Entity::find().paginate(db, query.page_size);
    let total = paginator.num_items().await?;
    let items: Vec<DemoDetails> = paginator
        .fetch_page(query.page - 1)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(ApiResponse::ok(PageResult::new(
        items,
        total,
        query.page,
        query.page_size,
    )))
}

pub async fn get(db: &DatabaseConnection, id: i64) -> ApplicationResult<DemoDetails> {
    let demo = Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| ApplicationError::NotFound(format!("Demo id: {id}")))?;
    Ok(ApiResponse::ok(demo.into()))
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
    Ok(ApiResponse::ok(demo.into()))
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
    active_model.name = Set(command.name);
    active_model.description = Set(command.description);

    let demo = active_model.update(db).await?.try_into_model()?;
    Ok(ApiResponse::ok(demo.into()))
}

pub async fn patch(
    db: &DatabaseConnection,
    command: PatchDemoCommand,
) -> ApplicationResult<DemoDetails> {
    let model = Entity::find_by_id(command.id)
        .one(db)
        .await?
        .ok_or_else(|| ApplicationError::NotFound(format!("Demo id: {}", command.id)))?;
    let mut active_model: ActiveModel = model.into();

    if let Some(name) = command.name {
        active_model.name = Set(name);
    }
    if let Some(description) = command.description {
        active_model.description = Set(Some(description));
    }

    let demo = active_model.update(db).await?.try_into_model()?;
    Ok(ApiResponse::ok(demo.into()))
}

pub async fn delete(db: &DatabaseConnection, id: i64) -> ApplicationResult<Empty> {
    let result = Entity::delete_by_id(id).exec(db).await?;
    if result.rows_affected == 0 {
        return Err(ApplicationError::NotFound(format!("Demo id: {id}")));
    }
    Ok(ApiResponse::empty_ok())
}
