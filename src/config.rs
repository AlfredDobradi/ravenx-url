use clap::Parser;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub verbose: bool,
    pub auth_token: String,
    pub key_version: String,
    pub otlp_endpoint: Option<String>,
    pub redis: Redis,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Redis {
    pub url: String,
}

pub fn load_config(path: String) -> Result<Config, anyhow::Error> {
    let raw = fs::read_to_string(&path)?;
    let cfg: Config = serde_yaml::from_str(&raw)?;

    Ok(cfg)
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short, long, default_value = "config.yaml")]
    pub config_path: String,
}
