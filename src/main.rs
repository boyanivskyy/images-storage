mod api;
mod db;
mod models;
mod services;

use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // db
    let pool = db::init_db().await?;
    let image_service = services::image_service::ImageService::new(pool);

    // server
    let app = api::routes::create_routes(image_service).layer(CorsLayer::permissive());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve::serve(listener, app).await.unwrap();

    Ok(())
}
