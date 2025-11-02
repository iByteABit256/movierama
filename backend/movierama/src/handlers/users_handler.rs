use crate::{models::Movie, services::user_service};
use axum::{Json, extract::Path};

pub async fn list_movies_by_user(Path(user_id): Path<i32>) -> Json<Vec<Movie>> {
    let movies = user_service::list_movies_by_user(user_id).await;
    Json(movies)
}
