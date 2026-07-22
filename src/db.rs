use crate::config::DatabaseConfig;
use infrastructure::migration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use tracing::info;

pub async fn init_db(database_config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(&database_config.url);

    options.min_connections(database_config.min_connections);
    options.max_connections(database_config.max_connections);

    let connection = Database::connect(options).await?;
    info!("数据库连接初始化成功");
    migration::run_migrations(&connection).await?;
    info!("数据库迁移完成");
    Ok(connection)
}
