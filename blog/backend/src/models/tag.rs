use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// A tag that can be attached to articles.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: OffsetDateTime,
}

/// Payload for creating a tag.
#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub slug: String,
}
