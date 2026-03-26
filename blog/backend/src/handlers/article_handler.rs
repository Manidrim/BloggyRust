use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::{AppError, AppResult},
    middleware::auth::AuthenticatedUser,
    models::article::{ArticleView, CreateArticleRequest, UpdateArticleRequest},
    repositories::article_repository::ArticleRepository,
    AppState,
};

/// GET /articles
pub async fn list_articles(State(state): State<AppState>) -> AppResult<Json<Vec<ArticleView>>> {
    let repo = ArticleRepository::new(&state.pool);
    let articles = repo.find_all_published().await?;
    Ok(Json(articles))
}

/// GET /articles/:slug
pub async fn get_article(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Json<ArticleView>> {
    let repo = ArticleRepository::new(&state.pool);
    let article = repo.find_by_slug(&slug).await?;
    Ok(Json(article))
}

/// POST /articles  (admin only)
pub async fn create_article(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateArticleRequest>,
) -> AppResult<(StatusCode, Json<ArticleView>)> {
    require_admin(&user)?;
    validate_create_payload(&payload)?;

    let repo = ArticleRepository::new(&state.pool);
    let slug = slugify(&payload.title);

    let article = repo
        .create(
            &payload.title,
            &slug,
            &payload.content,
            payload.excerpt.as_deref(),
            payload.published.unwrap_or(false),
            user.id,
        )
        .await?;

    if let Some(tag_ids) = &payload.tag_ids {
        repo.set_tags(article.id, tag_ids).await?;
    }

    let view = repo.find_by_slug(&article.slug).await?;
    Ok((StatusCode::CREATED, Json(view)))
}

/// PUT /articles/:slug  (admin only)
pub async fn update_article(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(slug): Path<String>,
    Json(payload): Json<UpdateArticleRequest>,
) -> AppResult<Json<ArticleView>> {
    require_admin(&user)?;

    let repo = ArticleRepository::new(&state.pool);
    let existing = repo.find_by_slug(&slug).await?;

    let updated = repo
        .update(
            existing.id,
            payload.title.as_deref(),
            payload.content.as_deref(),
            payload.excerpt.as_deref(),
            payload.published,
        )
        .await?;

    if let Some(tag_ids) = &payload.tag_ids {
        repo.set_tags(updated.id, tag_ids).await?;
    }

    let view = repo.find_by_slug(&updated.slug).await?;
    Ok(Json(view))
}

/// DELETE /articles/:slug  (admin only)
pub async fn delete_article(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(slug): Path<String>,
) -> AppResult<StatusCode> {
    require_admin(&user)?;

    let repo = ArticleRepository::new(&state.pool);
    let article = repo.find_by_slug(&slug).await?;
    repo.delete(article.id).await?;

    Ok(StatusCode::NO_CONTENT)
}

fn require_admin(user: &AuthenticatedUser) -> AppResult<()> {
    if user.is_admin {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

fn validate_create_payload(payload: &CreateArticleRequest) -> AppResult<()> {
    if payload.title.trim().is_empty() {
        return Err(AppError::BadRequest("Title cannot be empty".to_string()));
    }
    if payload.content.trim().is_empty() {
        return Err(AppError::BadRequest("Content cannot be empty".to_string()));
    }
    Ok(())
}

/// Converts a title into a URL-friendly slug.
fn slugify(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
