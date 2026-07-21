use sea_orm_migration::sea_orm::DatabaseConnection;
pub use sea_orm_migration::{MigratorTrait, prelude::*};

mod m20260721_000001_create_demo;

pub struct Migrator;

#[sea_orm_migration::async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260721_000001_create_demo::Migration)]
    }
}

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await
}
