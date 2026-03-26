use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// A registered user, as stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// Public view of a user (no sensitive fields).
#[derive(Debug, Serialize)]
pub struct UserView {
    pub id: Uuid,
    pub username: String,
    pub is_admin: bool,
}

impl From<User> for UserView {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            is_admin: user.is_admin,
        }
    }
}

/// Payload for user registration.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Payload for user login.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Successful authentication response.
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserView,
}
