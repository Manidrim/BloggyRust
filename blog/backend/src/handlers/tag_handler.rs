use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    middleware::auth::AuthenticatedUser,
    models::tag::{CreateTagRequest, Tag},
    repositories::tag_repository::TagRepository,
    AppState,
};

/// GET /tags
pub async fn list_tags(State(state): State<AppState>) -> AppResult<Json<Vec<Tag>>> {
    let repo = TagRepository::new(&state.pool);
    let tags = repo.find_all().await?;
    Ok(Json(tags))
}

/// POST /tags  (admin only)
pub async fn create_tag(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateTagRequest>,
) -> AppResult<(StatusCode, Json<Tag>)> {
    require_admin(&user)?;

    let repo = TagRepository::new(&state.pool);
    let tag = repo.create(&payload.name, &payload.slug).await?;
    Ok((StatusCode::CREATED, Json(tag)))
}

/// DELETE /tags/:id  (admin only)
pub async fn delete_tag(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    require_admin(&user)?;

    let repo = TagRepository::new(&state.pool);
    repo.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

fn require_admin(user: &AuthenticatedUser) -> AppResult<()> {
    if user.is_admin {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}
