use crate::blogs::models::{Article, ArticleQuery, CreateArticle, UpdateArticle, UpsertArticle};
use crate::common::DbPool;
use sqlx::Error;

pub async fn latest_articles(pool: &DbPool, limit: i64) -> Result<Vec<Article>, Error> {
    sqlx::query_as("SELECT * FROM blogs ORDER BY id DESC LIMIT $1")
        .bind(limit)
        .fetch_all(pool)
        .await
}

pub async fn list_articles(pool: &DbPool, query: ArticleQuery) -> Result<Vec<Article>, Error> {
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let offset = query.offset.unwrap_or(0).max(0);

    match query.folder_id {
        Some(folder_id) => {
            sqlx::query_as(
                r#"
                SELECT * FROM blogs
                WHERE folder_id = $1
                ORDER BY is_pinned DESC, created_at DESC, id DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(folder_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
        }
        None => {
            sqlx::query_as(
                r#"
                SELECT * FROM blogs
                ORDER BY is_pinned DESC, created_at DESC, id DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
        }
    }
}

pub async fn get_article(pool: &DbPool, id: i64) -> Result<Option<Article>, Error> {
    sqlx::query_as("SELECT * FROM blogs WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create_article(pool: &DbPool, article: CreateArticle) -> Result<i64, Error> {
    let id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO blogs (
            folder_id,
            title,
            description,
            cover_url,
            author,
            content,
            is_pinned
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7)
        RETURNING id
        "#,
    )
    .bind(article.folder_id)
    .bind(&article.title)
    .bind(&article.description)
    .bind(&article.cover_url)
    .bind(&article.author)
    .bind(&article.content)
    .bind(article.is_pinned.unwrap_or(false))
    .fetch_one(pool)
    .await?;

    sync_article_tags(pool, id, article.tag_ids.as_deref()).await?;
    Ok(id)
}

pub async fn update_article(
    pool: &DbPool,
    id: i64,
    article: UpdateArticle,
) -> Result<Option<i64>, Error> {
    let updated_id = sqlx::query_scalar(
        r#"
        UPDATE blogs
        SET
            folder_id = $1,
            title = $2,
            description = $3,
            cover_url = $4,
            author = $5,
            content = $6,
            is_pinned = $7
        WHERE id = $8
        RETURNING id
        "#,
    )
    .bind(article.folder_id)
    .bind(&article.title)
    .bind(&article.description)
    .bind(&article.cover_url)
    .bind(&article.author)
    .bind(&article.content)
    .bind(article.is_pinned.unwrap_or(false))
    .bind(id)
    .fetch_optional(pool)
    .await?;

    if updated_id.is_some() {
        sync_article_tags(pool, id, article.tag_ids.as_deref()).await?;
    }

    Ok(updated_id)
}

pub async fn delete_article(pool: &DbPool, id: i64) -> Result<u64, Error> {
    let result = sqlx::query("DELETE FROM blogs WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub async fn upsert(pool: &DbPool, upsert: UpsertArticle) -> Result<i64, Error> {
    match upsert.id {
        None => {
            create_article(
                pool,
                CreateArticle {
                    folder_id: upsert.folder_id,
                    title: upsert.title,
                    description: upsert.description,
                    cover_url: upsert.cover_url,
                    author: upsert.author,
                    content: upsert.content,
                    is_pinned: upsert.is_pinned,
                    tag_ids: upsert.tag_ids,
                },
            )
            .await
        }
        Some(id) => update_article(
            pool,
            id,
            UpdateArticle {
                folder_id: upsert.folder_id,
                title: upsert.title,
                description: upsert.description,
                cover_url: upsert.cover_url,
                author: upsert.author,
                content: upsert.content,
                is_pinned: upsert.is_pinned,
                tag_ids: upsert.tag_ids,
            },
        )
        .await?
        .ok_or(Error::RowNotFound),
    }
}

async fn sync_article_tags(
    pool: &DbPool,
    article_id: i64,
    tag_ids: Option<&[i64]>,
) -> Result<(), Error> {
    if let Some(tag_ids) = tag_ids {
        sqlx::query("DELETE FROM blog_article_tags WHERE article_id = $1")
            .bind(article_id)
            .execute(pool)
            .await?;

        for tag_id in tag_ids {
            sqlx::query(
                r#"
                INSERT INTO blog_article_tags (article_id, tag_id)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING
                "#,
            )
            .bind(article_id)
            .bind(tag_id)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
