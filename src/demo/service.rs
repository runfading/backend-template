use super::models::{ActiveModel as DemoModel, Entity as DemoEntity};
use crate::common::{ApiResponse, AppResult, DbPool, Empty};
use crate::demo::models::{CreateDemo, SimpleDemo, UpdateDemo};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TryIntoModel};

pub async fn list(pool: &DbPool) -> AppResult<Vec<SimpleDemo>> {
    Ok(ApiResponse::ok(
        DemoEntity::find()
            .all(pool)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    ))
}

pub async fn get(pool: &DbPool, id: i64) -> AppResult<SimpleDemo> {
    let model = DemoEntity::find_by_id(id)
        .one(pool)
        .await?
        .ok_or_else(|| crate::error::AppError::NotFound(format!("Demo not found: {}", id)))?;

    Ok(ApiResponse::ok(SimpleDemo {
        name: model.name,
        id: model.id,
    }))
}

pub async fn create(pool: &DbPool, input: CreateDemo) -> AppResult<SimpleDemo> {
    let now = time::OffsetDateTime::now_utc();
    let mut active_model = DemoModel {
        name: Set(input.name),
        created_at: Set(now.into()),
        ..Default::default()
    };

    if let Some(v) = input.description {
        active_model.description = Set(Some(v));
    }

    let model = active_model.save(pool).await?.try_into_model()?;

    Ok(ApiResponse::ok(SimpleDemo {
        name: model.name,
        id: model.id,
    }))
}

pub async fn update(pool: &DbPool, input: UpdateDemo) -> AppResult<SimpleDemo> {
    let mut active_model = DemoModel {
        id: Set(input.id),
        ..Default::default()
    };

    if let Some(v) = input.name {
        active_model.name = Set(v);
    }
    if let Some(v) = input.description {
        active_model.description = Set(Some(v));
    }

    let model = active_model.update(pool).await?.try_into_model()?;

    Ok(ApiResponse::ok(SimpleDemo {
        name: model.name,
        id: model.id,
    }))
}

pub async fn delete(pool: &DbPool, id: i64) -> AppResult<Empty> {
    let model = DemoEntity::find_by_id(id)
        .one(pool)
        .await?
        .ok_or_else(|| crate::error::AppError::NotFound(format!("Demo not found id: {}", id)))?;

    let active_model: super::models::ActiveModel = model.into();
    active_model.delete(pool).await?;

    Ok(ApiResponse::empty_ok())
}
