use tokio::test;

use crate::Sender::send_all;
#[allow(unused)]
use crate::{
    Config::get_config, MatrixBot::MatrixBot, Sender::BotSender, TgBot, bots::Bots::BotEnum,
};

#[test]
async fn test_init_csv() {
    use crate::{Config::get_config, Db::init};
    let cfg = get_config();
    dbg!(cfg);
    init().await.unwrap();
}

#[test]
async fn test_survey_api() {
    //保证服务器开启
}

#[test]
async fn test_matrix_bot() {
    let _ = MatrixBot::new().await;
    tokio::signal::ctrl_c().await.unwrap();
}

#[test]
async fn test_tg_bot() {
    let token = get_config().telegram_bot.clone().unwrap().token.clone();

    let _ = TgBot::TelegramBot::new(token.clone()).await;
    let bots = BotEnum::get().await.lock().await;
    let _ = dbg!(bots);
    send_all("test".to_string()).await;
    //tokio::signal::ctrl_c().await.unwrap();
}

#[test]
async fn test_output() {
    let md = format!(
        r#"# 新问卷来啦
**填写时间**：
**提交时间**：

-----  



-----  
  
"#
    );
    md.to_string();
    print!("{md}\n");
}
