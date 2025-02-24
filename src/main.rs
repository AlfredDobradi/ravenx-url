use axum::routing::delete;
use axum::{
    routing::{get, post},
    Router,
};
use tracing::info;
use tracing::log::LevelFilter;
use ravenx_url::{api, config};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let subscriber = FmtSubscriber::builder()
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber)?;

    let app = Router::new()
        .route("/{url_key}", get(api::handlers::handle_redirect));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn placeholder() -> &'static str {
    "hi"
}
