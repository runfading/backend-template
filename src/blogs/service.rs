use crate::blogs::models::{
    Article, ArticleQuery, CreateArticle, LatestArticles, UpdateArticle, UpsertArticle,
};
use crate::blogs::repo;
use crate::common::{ApiResponse, AppResult, DbPool};
use crate::error::AppError;

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

pub async fn list_articles(pool: &DbPool, query: ArticleQuery) -> AppResult<Vec<Article>> {
    Ok(ApiResponse::ok(repo::list_articles(pool, query).await?))
}

pub async fn get_article(pool: &DbPool, id: i64) -> AppResult<Article> {
    Ok(ApiResponse::ok(
        repo::get_article(pool, id)
            .await?
            .ok_or(AppError::NotFound)?,
    ))
}

pub async fn create_article(pool: &DbPool, article: CreateArticle) -> AppResult<i64> {
    Ok(ApiResponse::ok(repo::create_article(pool, article).await?))
}

pub async fn update_article(pool: &DbPool, id: i64, article: UpdateArticle) -> AppResult<i64> {
    Ok(ApiResponse::ok(
        repo::update_article(pool, id, article)
            .await?
            .ok_or(AppError::NotFound)?,
    ))
}

pub async fn delete_article(pool: &DbPool, id: i64) -> AppResult<()> {
    if repo::delete_article(pool, id).await? == 0 {
        return Err(AppError::NotFound);
    }
    Ok(ApiResponse::ok(()))
}

pub async fn upsert_articles(pool: &DbPool, upsert: UpsertArticle) -> AppResult<i64> {
    Ok(ApiResponse::ok(repo::upsert(pool, upsert).await?))
}
