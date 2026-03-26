use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::AppError, AppState};

/// Claims embedded in a JWT token.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub username: String,
    pub is_admin: bool,
    pub exp: usize,
}

/// Authenticated user extracted directly from the request headers.
///
/// Use this as a handler parameter to require authentication:
/// ```ignore
/// async fn my_handler(user: AuthenticatedUser) -> ... { }
/// ```
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub username: String,
    pub is_admin: bool,
}

impl From<JwtClaims> for AuthenticatedUser {
    fn from(claims: JwtClaims) -> Self {
        Self {
            id: claims.sub,
            username: claims.username,
            is_admin: claims.is_admin,
        }
    }
}

/// Axum extractor: validates the `Authorization: Bearer <token>` header
/// and produces an [`AuthenticatedUser`]. Returns 401 on failure.
#[axum::async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or(AppError::Unauthorized)?;

        let key = DecodingKey::from_secret(state.config.jwt_secret.as_bytes());
        let data = decode::<JwtClaims>(token, &key, &Validation::default())
            .map_err(|_| AppError::Unauthorized)?;

        Ok(AuthenticatedUser::from(data.claims))
    }
}

/// Creates a signed JWT for the given user.
pub fn create_jwt(
    user_id: Uuid,
    username: &str,
    is_admin: bool,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize
        + 7 * 24 * 3600; // 7 days

    let claims = JwtClaims {
        sub: user_id,
        username: username.to_owned(),
        is_admin,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
