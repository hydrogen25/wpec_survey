use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::OnceCell;

use crate::Sender::BotSender;
#[allow(non_snake_case)]
pub mod ApiServer;
#[allow(non_snake_case)]
pub mod Config;
#[allow(non_snake_case)]
pub mod Db;
#[allow(non_snake_case)]
pub mod Structs;
pub mod tests;

pub mod bots;
pub use bots::MatrixBot;
pub use bots::TgBot;
#[allow(non_snake_case)]
pub mod Sender;
