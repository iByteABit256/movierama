use crate::{auth::Claims, exceptions::MovieramaError, models::VoteType, services::vote_service};
use axum::{Json, extract::State};
use std::collections::HashMap;

/// POST /votes/user-votes
pub async fn get_user_votes(
    claims: Claims,
    State(pool): State<sqlx::PgPool>,
    Json(movie_ids): Json<Vec<i32>>,
) -> Result<Json<HashMap<i32, VoteType>>, MovieramaError> {
    let votes = vote_service::get_user_votes_for_movies(&pool, claims.user_id, &movie_ids).await?;
    Ok(Json(votes))
}
