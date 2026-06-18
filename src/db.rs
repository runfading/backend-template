use crate::common::DbPool;
use crate::config::DatabaseConfig;
use crate::error::AppError;
use sea_orm::{ConnectOptions, Database};
use tracing::info;

pub async fn init_db(database_config: &DatabaseConfig) -> Result<DbPool, AppError> {
    let mut options = ConnectOptions::new(&database_config.url);

    options.max_connections(database_config.max_connections);
    options.min_connections(database_config.min_connections);
    let connection = Database::connect(options).await?;
    info!("数据库连接初始化成功");
    Ok(connection)
}
