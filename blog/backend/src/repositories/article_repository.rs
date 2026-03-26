use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::article::{Article, ArticleView},
    models::tag::Tag,
};

/// Intermediate row returned by JOINed queries (article + author).
#[derive(sqlx::FromRow)]
struct ArticleRow {
    id: Uuid,
    title: String,
    slug: String,
    content: String,
    excerpt: Option<String>,
    author_id: Uuid,
    author_username: String,
    published: bool,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl ArticleRow {
    fn into_view(self, tags: Vec<Tag>) -> ArticleView {
        ArticleView {
            id: self.id,
            title: self.title,
            slug: self.slug,
            content: self.content,
            excerpt: self.excerpt,
            author_id: self.author_id,
            author_username: self.author_username,
            published: self.published,
            tags,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

pub struct ArticleRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> ArticleRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all_published(&self) -> AppResult<Vec<ArticleView>> {
        let rows = sqlx::query_as::<_, ArticleRow>(
            r#"
            SELECT
                a.id, a.title, a.slug, a.content, a.excerpt,
                a.author_id, u.username AS author_username,
                a.published, a.created_at, a.updated_at
            FROM articles a
            JOIN users u ON u.id = a.author_id
            WHERE a.published = TRUE
            ORDER BY a.created_at DESC
            "#,
        )
        .fetch_all(self.pool)
        .await?;

        let mut views = Vec::with_capacity(rows.len());
        for row in rows {
            let tags = self.find_tags_for_article(row.id).await?;
            views.push(row.into_view(tags));
        }
        Ok(views)
    }

    pub async fn find_by_slug(&self, slug: &str) -> AppResult<ArticleView> {
        let row = sqlx::query_as::<_, ArticleRow>(
            r#"
            SELECT
                a.id, a.title, a.slug, a.content, a.excerpt,
                a.author_id, u.username AS author_username,
                a.published, a.created_at, a.updated_at
            FROM articles a
            JOIN users u ON u.id = a.author_id
            WHERE a.slug = $1
            "#,
        )
        .bind(slug)
        .fetch_optional(self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Article '{slug}' not found")))?;

        let tags = self.find_tags_for_article(row.id).await?;
        Ok(row.into_view(tags))
    }

    pub async fn create(
        &self,
        title: &str,
        slug: &str,
        content: &str,
        excerpt: Option<&str>,
        published: bool,
        author_id: Uuid,
    ) -> AppResult<Article> {
        let article = sqlx::query_as::<_, Article>(
            r#"
            INSERT INTO articles (title, slug, content, excerpt, published, author_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(title)
        .bind(slug)
        .bind(content)
        .bind(excerpt)
        .bind(published)
        .bind(author_id)
        .fetch_one(self.pool)
        .await?;
        Ok(article)
    }

    pub async fn update(
        &self,
        id: Uuid,
        title: Option<&str>,
        content: Option<&str>,
        excerpt: Option<&str>,
        published: Option<bool>,
    ) -> AppResult<Article> {
        let article = sqlx::query_as::<_, Article>(
            r#"
            UPDATE articles SET
                title      = COALESCE($2, title),
                content    = COALESCE($3, content),
                excerpt    = COALESCE($4, excerpt),
                published  = COALESCE($5, published),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(title)
        .bind(content)
        .bind(excerpt)
        .bind(published)
        .fetch_optional(self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Article {id} not found")))?;
        Ok(article)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM articles WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Article {id} not found")));
        }
        Ok(())
    }

    pub async fn set_tags(&self, article_id: Uuid, tag_ids: &[Uuid]) -> AppResult<()> {
        sqlx::query("DELETE FROM article_tags WHERE article_id = $1")
            .bind(article_id)
            .execute(self.pool)
            .await?;

        for tag_id in tag_ids {
            sqlx::query(
                "INSERT INTO article_tags (article_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(article_id)
            .bind(tag_id)
            .execute(self.pool)
            .await?;
        }
        Ok(())
    }

    async fn find_tags_for_article(&self, article_id: Uuid) -> AppResult<Vec<Tag>> {
        let tags = sqlx::query_as::<_, Tag>(
            r#"
            SELECT t.id, t.name, t.slug, t.created_at
            FROM tags t
            JOIN article_tags at ON at.tag_id = t.id
            WHERE at.article_id = $1
            ORDER BY t.name
            "#,
        )
        .bind(article_id)
        .fetch_all(self.pool)
        .await?;
        Ok(tags)
    }
}
