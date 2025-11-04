use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

mod auth;
mod exceptions;
mod handlers;
mod models;
mod pagination;
mod routes;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = routes::create_router(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    tracing::info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
