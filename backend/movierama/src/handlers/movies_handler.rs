use crate::{
    exceptions::MovieramaError,
    models::{Movie, NewMovie},
    services::movie_service,
};
use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::{Value, json};
use sqlx::PgPool;

/// GET /movies
pub async fn list_movies(State(pool): State<PgPool>) -> Result<Json<Vec<Movie>>, MovieramaError> {
    let movies = movie_service::list_all_movies(&pool).await?;
    Ok(Json(movies))
}

/// GET /movies/{movie_id}
pub async fn get_movie(
    State(pool): State<PgPool>,
    Path(movie_id): Path<i32>,
) -> Result<Json<Movie>, MovieramaError> {
    let movie = movie_service::get_movie_by_id(&pool, movie_id).await?;
    match movie {
        Some(m) => Ok(Json(m)),
        None => Err(MovieramaError::NotFound),
    }
}

/// UPDATE /movies/{movie_id}
pub async fn update_movie(
    State(pool): State<PgPool>,
    Path(movie_id): Path<i32>,
    Json(payload): Json<NewMovie>,
) -> Result<Json<NewMovie>, MovieramaError> {
    let movie = movie_service::update_movie(&pool, movie_id, payload).await?;
    Ok(Json(movie))
}

/// POST /movies
pub async fn create_movie(
    State(pool): State<PgPool>,
    Json(payload): Json<NewMovie>,
) -> Result<Json<Movie>, MovieramaError> {
    let movie = movie_service::create_movie(&pool, payload).await?;
    Ok(Json(movie))
}

/// DELETE /movies/{movie_id}
pub async fn delete_movie(
    State(pool): State<PgPool>,
    Path(movie_id): Path<i32>,
) -> Result<Json<Value>, MovieramaError> {
    let success = movie_service::delete_movie(&pool, movie_id).await?;
    if success {
        Ok(Json(json!(format!(
            "Movie with id {} deleted successfully",
            movie_id
        ))))
    } else {
        Err(MovieramaError::NotFound)
    }
}
