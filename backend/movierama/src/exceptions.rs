use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MovieramaError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Movie not found")]
    NotFound,
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

impl IntoResponse for MovieramaError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            MovieramaError::DatabaseError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            MovieramaError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            MovieramaError::UnexpectedError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}
