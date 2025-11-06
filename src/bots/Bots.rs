use crate::Config::get_config;
use crate::TgBot;
use crate::{MatrixBot::MatrixBot, TgBot::TelegramBot};
use tokio::sync::Mutex;
use tokio::sync::OnceCell;

pub async fn init_bots() {
    let token = get_config().telegram_bot.clone().unwrap().token.clone();

    let _ = TgBot::TelegramBot::new(token.clone()).await;

    let _ = MatrixBot::new().await;
}

#[derive(Debug, Clone)]
pub enum BotEnum {
    MatrixBot(MatrixBot),
    TelegramBot(TelegramBot),
}

static REG_BOTS: OnceCell<Mutex<Vec<BotEnum>>> = OnceCell::const_new();

impl BotEnum {
    pub async fn get() -> &'static Mutex<Vec<BotEnum>> {
        REG_BOTS
            .get_or_init(|| async move { Mutex::new(Vec::new()) })
            .await
    }

    pub async fn push_bot(bot: BotEnum) {
        let mut list = BotEnum::get().await.lock().await;

        list.push(bot);
    }
}
