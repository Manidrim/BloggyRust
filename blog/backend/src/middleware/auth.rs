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

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{decode, DecodingKey, Validation};

    #[test]
    fn create_jwt_produces_decodable_token_with_correct_claims() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let user_id = Uuid::new_v4();
        let secret = "test_secret_key";

        let token = create_jwt(user_id, "alice", true, secret)
            .expect("JWT creation should succeed");

        let key = DecodingKey::from_secret(secret.as_bytes());
        let mut validation = Validation::default();
        validation.validate_exp = false;
        let decoded = decode::<JwtClaims>(&token, &key, &validation)
            .expect("JWT decoding should succeed");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        assert!(decoded.claims.exp > now, "exp should be set in the future");

        assert_eq!(decoded.claims.sub, user_id);
        assert_eq!(decoded.claims.username, "alice");
        assert!(decoded.claims.is_admin);
    }

    #[test]
    fn create_jwt_with_different_secret_fails_to_decode() {
        use jsonwebtoken::errors::ErrorKind;
        let user_id = Uuid::new_v4();
        let token = create_jwt(user_id, "bob", false, "secret_a")
            .expect("JWT creation should succeed");

        let key = DecodingKey::from_secret("secret_b".as_bytes());
        let result = decode::<JwtClaims>(&token, &key, &Validation::default());

        assert!(
            matches!(result.unwrap_err().kind(), ErrorKind::InvalidSignature),
            "expected InvalidSignature error when using a different secret"
        );
    }

    #[test]
    fn create_jwt_non_admin_claim_is_preserved() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret";
        let token = create_jwt(user_id, "bob", false, secret)
            .expect("JWT creation should succeed");

        let key = DecodingKey::from_secret(secret.as_bytes());
        let mut validation = Validation::default();
        validation.validate_exp = false;
        let decoded = decode::<JwtClaims>(&token, &key, &validation)
            .expect("JWT decoding should succeed");

        assert!(!decoded.claims.is_admin);
        assert_eq!(decoded.claims.username, "bob");
    }
}
