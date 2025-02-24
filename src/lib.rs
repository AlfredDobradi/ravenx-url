use lazy_static::lazy_static;
use tracing::info;

pub mod api;

pub mod config;

lazy_static! {
    pub static ref CONFIG: config::Config = {
        config::load_config().expect("Failed to load config")
    };
}