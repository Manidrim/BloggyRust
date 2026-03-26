use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handlers::{
        article_handler::{create_article, delete_article, get_article, list_articles, update_article},
        auth_handler::{login, register},
        comment_handler::{create_comment, delete_comment, list_comments},
        tag_handler::{create_tag, delete_tag, list_tags},
    },
    AppState,
};

/// Builds the complete API router.
///
/// Auth is handled per-handler via the [`AuthenticatedUser`] extractor —
/// no middleware layer needed, no overlapping route conflicts.
pub fn api_router(state: AppState) -> Router {
    Router::new()
        // Auth
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        // Articles (GET = public, POST/PUT/DELETE = admin via extractor)
        .route("/articles", get(list_articles).post(create_article))
        .route("/articles/:slug", get(get_article).put(update_article).delete(delete_article))
        // Comments (GET = public, POST = authenticated, DELETE = author/admin)
        .route("/articles/:id/comments", get(list_comments).post(create_comment))
        .route("/comments/:id", delete(delete_comment))
        // Tags (GET = public, POST/DELETE = admin)
        .route("/tags", get(list_tags).post(create_tag))
        .route("/tags/:id", delete(delete_tag))
        .with_state(state)
}
