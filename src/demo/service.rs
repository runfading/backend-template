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
            .map(|item| SimpleDemo {
                name: item.title,
                id: item.id,
            })
            .collect(),
    ))
}

pub async fn get(pool: &DbPool, id: i64) -> AppResult<SimpleDemo> {
    let model = DemoEntity::find_by_id(id)
        .one(pool)
        .await?
        .ok_or_else(|| crate::error::AppError::NotFound(format!("Demo not found: {}", id)))?;

    Ok(ApiResponse::ok(SimpleDemo {
        name: model.title,
        id: model.id,
    }))
}

pub async fn create(pool: &DbPool, input: CreateDemo) -> AppResult<SimpleDemo> {
    let now = chrono::Utc::now().with_timezone(&chrono::Local);
    let active_model = DemoModel {
        folder_id: Set(input.folder_id),
        title: Set(input.title),
        description: Set(input.description),
        cover_url: Set(input.cover_url),
        author: Set(input.author),
        content: Set(input.content),
        is_pinned: Set(false),
        created_at: Set(now.into()),
        ..Default::default()
    };

    let model = active_model.save(pool).await?.try_into_model()?;

    Ok(ApiResponse::ok(SimpleDemo {
        name: model.title,
        id: model.id,
    }))
}

pub async fn update(pool: &DbPool, input: UpdateDemo) -> AppResult<SimpleDemo> {
    let mut active_model = DemoModel {
        id: Set(input.id),
        ..Default::default()
    };

    if let Some(v) = input.folder_id {
        active_model.folder_id = Set(v);
    }
    if let Some(v) = input.title {
        active_model.title = Set(v);
    }
    if let Some(v) = input.description {
        active_model.description = Set(v);
    }
    if let Some(v) = input.cover_url {
        active_model.cover_url = Set(Some(v));
    }
    if let Some(v) = input.author {
        active_model.author = Set(v);
    }
    if let Some(v) = input.content {
        active_model.content = Set(v);
    }
    if let Some(v) = input.is_pinned {
        active_model.is_pinned = Set(v);
    }

    let model = active_model.update(pool).await?.try_into_model()?;

    Ok(ApiResponse::ok(SimpleDemo {
        name: model.title,
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
