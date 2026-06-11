use crate::blogs::models::LatestArticles;
use crate::blogs::repo;
use crate::common::{ApiResponse, AppResult, DbPool};

pub async fn latest_articles(pool: &DbPool) -> AppResult<Vec<LatestArticles>> {
    let vec = repo::latest_articles(pool, 3).await?;
    Ok(ApiResponse::ok(
        vec.into_iter()
            .map(|item| LatestArticles {
                name: item.title,
                id: item.id,
            })
            .collect(),
    ))
}
