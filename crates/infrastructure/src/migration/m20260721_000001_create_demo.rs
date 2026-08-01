use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[sea_orm_migration::async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Demo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Demo::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Demo::Name).string().not_null())
                    .col(ColumnDef::new(Demo::Description).text())
                    .col(
                        ColumnDef::new(Demo::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Demo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Demo {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
}
