use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// A comment on an article.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub article_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// Comment enriched with the author's username.
#[derive(Debug, Serialize)]
pub struct CommentView {
    pub id: Uuid,
    pub article_id: Uuid,
    pub author_id: Uuid,
    pub author_username: String,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// Payload for posting a comment.
#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}
