use std::collections::BTreeMap;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub urls: BTreeMap<String, Url>
}

#[derive(Debug, Deserialize)]
pub struct Url {
    pub url: String
}

pub fn load_config() -> Result<Config, anyhow::Error> {
    let raw= fs::read_to_string("config.yaml")?;
    let cfg: Config = serde_yaml::from_str(&raw)?;

    Ok(cfg)
}