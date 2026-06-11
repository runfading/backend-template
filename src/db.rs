use crate::common::DbPool;
use crate::error::AppError;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

pub async fn init_db(database_url: &str) -> Result<DbPool, AppError> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    info!("数据库连接完成");
    Ok(pool)
}
