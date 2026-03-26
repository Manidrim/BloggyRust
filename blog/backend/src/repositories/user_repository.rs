use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::user::User,
};

pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User {id} not found")))
    }

    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(self.pool)
            .await?;
        Ok(user)
    }

    pub async fn email_exists(&self, email: &str) -> AppResult<bool> {
        let row: (bool,) =
            sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
                .bind(email)
                .fetch_one(self.pool)
                .await?;
        Ok(row.0)
    }

    pub async fn username_exists(&self, username: &str) -> AppResult<bool> {
        let row: (bool,) =
            sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
                .bind(username)
                .fetch_one(self.pool)
                .await?;
        Ok(row.0)
    }

    pub async fn create(&self, username: &str, email: &str, password_hash: &str) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn create_admin(&self, username: &str, email: &str, password_hash: &str) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, password_hash, is_admin) VALUES ($1, $2, $3, TRUE) RETURNING *",
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(self.pool)
        .await?;
        Ok(user)
    }
}
