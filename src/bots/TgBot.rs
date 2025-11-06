use teloxide::{
    dispatching::dialogue::GetChatId,
    prelude::*,
    types::Me,
    utils::command::{self, BotCommands},
};

use crate::{Config::get_config, Sender::BotSender, bots::Bots::BotEnum};

#[derive(Debug, Clone)]
pub struct TelegramBot {
    pub bot: Bot,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "检测机器人是否存活")]
    Ping,
}

impl TelegramBot {
    pub async fn new(token: String) -> Self {
        let bot = Bot::new(token);

        let s = Self { bot };
        s.clone().run().await;

        s
    }

    pub async fn run(self) {
        let bot_clone = self.bot.clone();
        tokio::spawn(async move { Command::repl(bot_clone, TelegramBot::answer).await });
        BotEnum::push_bot(BotEnum::TelegramBot(self)).await;
    }

    async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        match cmd {
            Command::Ping => {
                bot.send_message(msg.chat.id, "Pong!").await?;
                dbg!(msg.chat.id);
            }
        };

        Ok(())
    }
}

impl BotSender for TelegramBot {
    async fn send(&self, message: String) -> anyhow::Result<()> {
        let config = get_config();
        let chat_id = config.telegram_bot.clone().unwrap().chat_id.clone();
        self.bot.send_message(chat_id, message).await?;
        Ok(())
    }
}
