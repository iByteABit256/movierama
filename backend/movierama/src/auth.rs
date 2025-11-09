use crate::exceptions::MovieramaError;
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub user_id: i32,
    pub exp: usize,
}

const JWT_SECRET: &str = "JWT_SECRET";

/// Axum extractor for protected routes
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = MovieramaError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Get the bearer token from the Authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| MovieramaError::Unauthorized)?;

        let jwt_secret = std::env::var(JWT_SECRET)
            .map_err(|e| MovieramaError::UnexpectedError(e.to_string()))?;

        // Decode the JWT token
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| MovieramaError::Unauthorized)?;

        // Check expiration
        let now = Utc::now().timestamp() as usize;
        if token_data.claims.exp < now {
            return Err(MovieramaError::Unauthorized);
        }

        Ok(token_data.claims)
    }
}
