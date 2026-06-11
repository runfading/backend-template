use crate::blogs::models::Article;
use crate::common::DbPool;
use sqlx::Error;

pub async fn latest_articles(pool: &DbPool, limit: i64) -> Result<Vec<Article>, Error> {
    sqlx::query_as("SELECT * FROM blogs ORDER BY id DESC LIMIT $1")
        .bind(limit)
        .fetch_all(pool)
        .await
}
