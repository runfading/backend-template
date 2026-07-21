use crate::dal::demo::models::{ActiveModel, Entity, Model};
use domain::biz::demo::domain::DemoRepository;
use domain::biz::demo::models::{CreateDemo, Demo, UpdateDemo};
use domain::error::DomainError;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TryIntoModel};

#[derive(Clone)]
pub struct SeaOrmDemoRepository {
    db: DatabaseConnection,
}

impl SeaOrmDemoRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl DemoRepository for SeaOrmDemoRepository {
    async fn list(&self) -> Result<Vec<Demo>, DomainError> {
        Entity::find()
            .all(&self.db)
            .await
            .map_err(to_repository_error)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Demo>, DomainError> {
        Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(to_repository_error)?
            .map(TryInto::try_into)
            .transpose()
    }

    async fn save(&self, demo: CreateDemo) -> Result<Demo, DomainError> {
        let active_model = ActiveModel {
            name: Set(demo.name),
            description: Set(demo.description),
            created_at: Set(time::OffsetDateTime::now_utc()),
            ..Default::default()
        };

        active_model
            .save(&self.db)
            .await
            .map_err(to_repository_error)?
            .try_into_model()
            .map_err(to_repository_error)?
            .try_into()
    }

    async fn update(&self, patch: UpdateDemo) -> Result<Demo, DomainError> {
        let mut active_model = ActiveModel {
            id: Set(patch.id.value()),
            ..Default::default()
        };

        if let Some(name) = patch.name {
            active_model.name = Set(name);
        }
        if let Some(description) = patch.description {
            active_model.description = Set(Some(description));
        }

        active_model
            .update(&self.db)
            .await
            .map_err(to_repository_error)?
            .try_into_model()
            .map_err(to_repository_error)?
            .try_into()
    }

    async fn delete(&self, id: i64) -> Result<(), DomainError> {
        let model = Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(to_repository_error)?
            .ok_or_else(|| DomainError::NotFound(format!("Demo not found id: {}", id)))?;

        let active_model: ActiveModel = model.into();
        active_model
            .delete(&self.db)
            .await
            .map_err(to_repository_error)?;

        Ok(())
    }
}

fn to_repository_error(err: sea_orm::DbErr) -> DomainError {
    DomainError::Repository(err.to_string())
}

impl TryFrom<Model> for Demo {
    type Error = DomainError;

    fn try_from(value: Model) -> Result<Self, Self::Error> {
        Demo::restore(value.id, value.name, value.description)
    }
}
