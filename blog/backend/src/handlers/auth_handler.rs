use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, Json};

use crate::{
    error::{AppError, AppResult},
    middleware::auth::create_jwt,
    models::user::{AuthResponse, LoginRequest, RegisterRequest, UserView},
    repositories::user_repository::UserRepository,
    AppState,
};

/// POST /auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    validate_register_payload(&payload)?;

    let repo = UserRepository::new(&state.pool);

    if repo.email_exists(&payload.email).await? {
        return Err(AppError::Conflict("Email already in use".to_string()));
    }
    if repo.username_exists(&payload.username).await? {
        return Err(AppError::Conflict("Username already taken".to_string()));
    }

    let password_hash = hash_password(&payload.password)?;
    let user = repo.create(&payload.username, &payload.email, &password_hash).await?;

    let token = create_jwt(user.id, &user.username, user.is_admin, &state.config.jwt_secret)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user: UserView::from(user),
    }))
}

/// POST /auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let repo = UserRepository::new(&state.pool);

    let user = repo
        .find_by_email(&payload.email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    verify_password(&payload.password, &user.password_hash)?;

    let token = create_jwt(user.id, &user.username, user.is_admin, &state.config.jwt_secret)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user: UserView::from(user),
    }))
}

fn validate_register_payload(payload: &RegisterRequest) -> AppResult<()> {
    if payload.username.trim().is_empty() {
        return Err(AppError::BadRequest("Username cannot be empty".to_string()));
    }
    if payload.password.len() < 8 {
        return Err(AppError::BadRequest("Password must be at least 8 characters".to_string()));
    }
    if !payload.email.contains('@') {
        return Err(AppError::BadRequest("Invalid email address".to_string()));
    }
    Ok(())
}

fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {e}")))
}

fn verify_password(password: &str, hash: &str) -> AppResult<()> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| AppError::Internal(format!("Invalid hash: {e}")))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)
}
