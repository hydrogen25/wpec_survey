use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::Structs::Problem;
use crate::TgBot::TelegramBot;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

pub static CFG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CFG.get_or_init(|| Config::load_from_file(Path::new("config.toml")).unwrap())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub host: String,
    pub data_position: Option<String>,
    pub api_address: String,
    #[serde(default)]
    pub problems: Vec<Problem>,
    pub matrix_bot: Option<matrix_bot>,
    pub telegram_bot: Option<telegram_bot>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_camel_case_types)]
pub struct matrix_bot {
    pub homeserver_url: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_camel_case_types)]
pub struct telegram_bot {
    pub token: String,
    pub chat_id: String,
}
impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
