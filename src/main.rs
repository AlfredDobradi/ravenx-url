use axum::{
    routing::get,
    Router,
};
use tracing::{info, Level};
use ravenx_url::{api, config};
use tracing_subscriber::FmtSubscriber;
use clap::Parser;
use ravenx_url::api::state::AppState;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = config::Args::parse();

    let cfg = config::load_config(args.config_path)?;

    let redis_client = redis::Client::open("redis://127.0.0.1:6379/")?;

    let max_level = match args.verbose || cfg.verbose {
        true => Level::DEBUG,
        false => Level::INFO,
    };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(max_level)
        .finish();

    let _ = tracing::subscriber::set_global_default(subscriber)?;

    let state = AppState::new(cfg, redis_client);
    let app = Router::new()
        .route("/{url_key}", get(api::handlers::handle_redirect))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
