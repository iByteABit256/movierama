use crate::{
    exceptions::MovieramaError,
    models::{AuthResponse, LoginUser, RegisterUser},
    services::auth_service,
};
use axum::{Json, extract::State};
use sqlx::PgPool;

/// POST /register
pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterUser>,
) -> Result<Json<AuthResponse>, MovieramaError> {
    let user = auth_service::register_user(&pool, &payload).await?;
    Ok(Json(user))
}

/// POST /login
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginUser>,
) -> Result<Json<AuthResponse>, MovieramaError> {
    let token = auth_service::login_user(&pool, &payload).await?;
    Ok(Json(token))
}
