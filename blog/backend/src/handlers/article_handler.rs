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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        middleware::auth::AuthenticatedUser,
        models::article::CreateArticleRequest,
    };
    use uuid::Uuid;

    fn admin_user() -> AuthenticatedUser {
        AuthenticatedUser {
            id: Uuid::new_v4(),
            username: "admin".to_string(),
            is_admin: true,
        }
    }

    fn regular_user() -> AuthenticatedUser {
        AuthenticatedUser {
            id: Uuid::new_v4(),
            username: "user".to_string(),
            is_admin: false,
        }
    }

    // --- slugify ---

    #[test]
    fn slugify_lowercases_and_replaces_spaces() {
        assert_eq!(slugify("Hello World"), "hello-world");
    }

    #[test]
    fn slugify_collapses_multiple_separators() {
        assert_eq!(slugify("Hello  --  World"), "hello-world");
    }

    #[test]
    fn slugify_removes_special_characters() {
        // é est alphanumeric en Rust (Unicode), donc conservé tel quel
        assert_eq!(slugify("C'est l'été !"), "c-est-l-été");
    }

    #[test]
    fn slugify_preserves_numbers() {
        assert_eq!(slugify("Article 42"), "article-42");
    }

    #[test]
    fn slugify_empty_string_returns_empty() {
        assert_eq!(slugify(""), "");
    }

    // --- validate_create_payload ---

    #[test]
    fn validate_create_rejects_empty_title() {
        let payload = CreateArticleRequest {
            title: "   ".to_string(),
            content: "some content".to_string(),
            excerpt: None,
            published: None,
            tag_ids: None,
        };
        let result = validate_create_payload(&payload);
        assert!(matches!(result, Err(AppError::BadRequest(ref msg)) if msg.to_lowercase().contains("title")));
    }

    #[test]
    fn validate_create_rejects_empty_content() {
        let payload = CreateArticleRequest {
            title: "My Title".to_string(),
            content: "   ".to_string(),
            excerpt: None,
            published: None,
            tag_ids: None,
        };
        let result = validate_create_payload(&payload);
        assert!(matches!(result, Err(AppError::BadRequest(ref msg)) if msg.to_lowercase().contains("content")));
    }

    #[test]
    fn validate_create_accepts_valid_payload() {
        let payload = CreateArticleRequest {
            title: "My Title".to_string(),
            content: "Some content".to_string(),
            excerpt: None,
            published: None,
            tag_ids: None,
        };
        let result = validate_create_payload(&payload);
        assert!(result.is_ok());
    }

    // --- require_admin ---

    #[test]
    fn require_admin_allows_admin_user() {
        assert!(require_admin(&admin_user()).is_ok());
    }

    #[test]
    fn require_admin_rejects_regular_user() {
        let result = require_admin(&regular_user());
        assert!(matches!(result, Err(AppError::Forbidden)));
    }
}
