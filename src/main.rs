use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};
use clap::Parser;
use ravenx_url::{api, config};
use ravenx_url::{api::state::AppState, instrumentation};
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = config::Args::parse();

    let cfg = config::load_config(args.config_path)?;

    let redis_client = redis::Client::open(cfg.redis.url.clone())?;

    let max_level = match args.verbose || cfg.verbose {
        true => Level::DEBUG,
        false => Level::INFO,
    };
    instrumentation::init_tracing_subscriber(max_level)?;

    let state = AppState::new(cfg, redis_client);

    let auth_routes = Router::new()
        .route("/{url_key}", post(api::handlers::handle_post))
        .route("/{url_key}", put(api::handlers::handle_put))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            api::middleware::auth_middleware,
        ));

    let app = Router::new()
        .route("/", get(api::handlers::handle_index))
        .route("/{url_key}", get(api::handlers::handle_redirect))
        .merge(auth_routes)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
