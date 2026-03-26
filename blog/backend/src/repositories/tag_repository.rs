use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::tag::Tag,
};

pub struct TagRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> TagRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> AppResult<Vec<Tag>> {
        let tags = sqlx::query_as::<_, Tag>("SELECT * FROM tags ORDER BY name")
            .fetch_all(self.pool)
            .await?;
        Ok(tags)
    }

    pub async fn find_by_slug(&self, slug: &str) -> AppResult<Tag> {
        sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE slug = $1")
            .bind(slug)
            .fetch_optional(self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Tag '{slug}' not found")))
    }

    pub async fn create(&self, name: &str, slug: &str) -> AppResult<Tag> {
        let tag = sqlx::query_as::<_, Tag>(
            "INSERT INTO tags (name, slug) VALUES ($1, $2) RETURNING *",
        )
        .bind(name)
        .bind(slug)
        .fetch_one(self.pool)
        .await?;
        Ok(tag)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM tags WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Tag {id} not found")));
        }
        Ok(())
    }
}
