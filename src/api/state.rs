use crate::config::Config;
use axum::extract::FromRef;
use redis::Client;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Config,
    pub redis: Client,
}

impl AppState {
    pub fn new(config: Config, redis: Client) -> Self {
        Self { config, redis }
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(input: &AppState) -> Self {
        input.config.clone()
    }
}

impl FromRef<AppState> for Client {
    fn from_ref(input: &AppState) -> Self {
        input.redis.clone()
    }
}
