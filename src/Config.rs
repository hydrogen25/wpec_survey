use anyhow::Result;
use serde::Deserialize;

use std::fs;
use std::path::Path;
use std::sync::OnceLock;
pub static CFG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CFG.get_or_init(|| Config::load_from_file(Path::new("config.toml")).unwrap())
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
