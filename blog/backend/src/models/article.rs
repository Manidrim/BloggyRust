use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use super::tag::Tag;

/// An article as stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub author_id: Uuid,
    pub published: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// Article enriched with its author name and associated tags.
#[derive(Debug, Serialize)]
pub struct ArticleView {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub author_id: Uuid,
    pub author_username: String,
    pub published: bool,
    pub tags: Vec<Tag>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// Payload for creating a new article.
#[derive(Debug, Deserialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub published: Option<bool>,
    pub tag_ids: Option<Vec<Uuid>>,
}

/// Payload for updating an existing article.
#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub published: Option<bool>,
    pub tag_ids: Option<Vec<Uuid>>,
}
