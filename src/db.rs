use crate::config::DatabaseConfig;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use tracing::info;

pub async fn init_db(database_config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(&database_config.url);

    options.max_connections(database_config.max_connections);
    options.min_connections(database_config.min_connections);
    let connection = Database::connect(options).await?;
    info!("数据库连接初始化成功");
    Ok(connection)
}
