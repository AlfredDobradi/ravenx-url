use axum::extract::FromRef;
use crate::config::Config;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Config
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(input: &AppState) -> Self {
        input.config.clone()
    }
}
