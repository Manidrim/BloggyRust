use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    error::AppResult,
    middleware::auth::AuthenticatedUser,
    models::comment::{CommentView, CreateCommentRequest},
    repositories::comment_repository::CommentRepository,
    AppState,
};

/// GET /articles/:article_id/comments
pub async fn list_comments(
    State(state): State<AppState>,
    Path(article_id): Path<Uuid>,
) -> AppResult<Json<Vec<CommentView>>> {
    let repo = CommentRepository::new(&state.pool);
    let comments = repo.find_by_article(article_id).await?;
    Ok(Json(comments))
}

/// POST /articles/:article_id/comments  (authenticated)
pub async fn create_comment(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(article_id): Path<Uuid>,
    Json(payload): Json<CreateCommentRequest>,
) -> AppResult<(StatusCode, Json<CommentView>)> {
    let repo = CommentRepository::new(&state.pool);
    let comment = repo.create(article_id, user.id, &payload.content).await?;
    Ok((StatusCode::CREATED, Json(comment)))
}

/// DELETE /comments/:comment_id  (author or admin)
pub async fn delete_comment(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(comment_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let repo = CommentRepository::new(&state.pool);
    repo.delete(comment_id, user.id, user.is_admin).await?;
    Ok(StatusCode::NO_CONTENT)
}
