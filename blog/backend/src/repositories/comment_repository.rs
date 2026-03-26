use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::comment::CommentView,
};

/// Intermediate row returned by JOINed queries (comment + author).
#[derive(sqlx::FromRow)]
struct CommentRow {
    id: Uuid,
    article_id: Uuid,
    author_id: Uuid,
    author_username: String,
    content: String,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl From<CommentRow> for CommentView {
    fn from(row: CommentRow) -> Self {
        Self {
            id: row.id,
            article_id: row.article_id,
            author_id: row.author_id,
            author_username: row.author_username,
            content: row.content,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

pub struct CommentRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> CommentRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_article(&self, article_id: Uuid) -> AppResult<Vec<CommentView>> {
        let views = sqlx::query_as::<_, CommentRow>(
            r#"
            SELECT
                c.id, c.article_id, c.author_id,
                u.username AS author_username,
                c.content, c.created_at, c.updated_at
            FROM comments c
            JOIN users u ON u.id = c.author_id
            WHERE c.article_id = $1
            ORDER BY c.created_at ASC
            "#,
        )
        .bind(article_id)
        .fetch_all(self.pool)
        .await?
        .into_iter()
        .map(CommentView::from)
        .collect();

        Ok(views)
    }

    pub async fn create(&self, article_id: Uuid, author_id: Uuid, content: &str) -> AppResult<CommentView> {
        let row = sqlx::query_as::<_, CommentRow>(
            r#"
            WITH inserted AS (
                INSERT INTO comments (article_id, author_id, content)
                VALUES ($1, $2, $3)
                RETURNING *
            )
            SELECT
                i.id, i.article_id, i.author_id,
                u.username AS author_username,
                i.content, i.created_at, i.updated_at
            FROM inserted i
            JOIN users u ON u.id = i.author_id
            "#,
        )
        .bind(article_id)
        .bind(author_id)
        .bind(content)
        .fetch_one(self.pool)
        .await?;

        Ok(CommentView::from(row))
    }

    pub async fn delete(&self, comment_id: Uuid, requester_id: Uuid, is_admin: bool) -> AppResult<()> {
        #[derive(sqlx::FromRow)]
        struct AuthorRow {
            author_id: Uuid,
        }

        let comment = sqlx::query_as::<_, AuthorRow>("SELECT author_id FROM comments WHERE id = $1")
            .bind(comment_id)
            .fetch_optional(self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Comment {comment_id} not found")))?;

        if !is_admin && comment.author_id != requester_id {
            return Err(AppError::Forbidden);
        }

        sqlx::query("DELETE FROM comments WHERE id = $1")
            .bind(comment_id)
            .execute(self.pool)
            .await?;

        Ok(())
    }
}
