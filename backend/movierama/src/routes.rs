use crate::handlers::movies_handler;
use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    let movie_routes = Router::new()
        .route(
            "/",
            get(movies_handler::list_movies).post(movies_handler::create_movie),
        )
        .route(
            "/{id}",
            get(movies_handler::get_movie)
                .delete(movies_handler::delete_movie)
                .put(movies_handler::update_movie),
        );

    Router::new()
        .nest("/api/v1/movies", movie_routes)
        .with_state(pool)
}
