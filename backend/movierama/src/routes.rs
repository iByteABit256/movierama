use crate::handlers::{auth_handler, movies_handler};
use axum::{
    Router,
    routing::{get, post},
};
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
        )
        .route("/{id}/vote", post(movies_handler::vote_movie));

    let auth_routes = Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login));

    Router::new()
        .nest("/api/v1/movies", movie_routes)
        .nest("/api/v1/auth", auth_routes)
        .with_state(pool)
}
