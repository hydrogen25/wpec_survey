use chrono::DateTime;
use chrono_tz::Asia::Shanghai;

use crate::{Structs::SurveyRequest, bots::Bots::BotEnum};

pub trait BotSender {
    fn send(&self, message: String)
    -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}

pub async fn send_all(msg: String) {
    let bots = BotEnum::get().await.lock().await.clone();
    dbg!(&bots);
    for botenum in bots.into_iter() {
        match botenum {
            BotEnum::TelegramBot(_b) => {
                // TODO: send via Telegram bot, e.g. _b.bot.send(msg.clone()).await;
                let _ = _b.send(msg.clone()).await;
            }
            BotEnum::MatrixBot(_b) => {
                // TODO: send via Matrix bot
                let _ = _b.send(msg.clone()).await;
            }
        };
    }
}

pub fn fmt_survey(sr: SurveyRequest) -> String {
    let end_time_human = chrono::Utc::now().with_timezone(&Shanghai).to_string();
    let start_time_human = chrono::DateTime::from_timestamp_secs(sr.start_time as i64)
        .unwrap_or(DateTime::from_timestamp_secs(0 as i64).unwrap())
        .to_string();

    let q_and_a = sr
        .data
        .iter()
        .map(|i| {
            format!(
                r#"
                
### 问题{}：{}  
回答：{}  "#,
                i.id,
                i.question,
                i.answer.clone().unwrap_or("空".to_string())
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let md = format!(
        r#"# 新问卷来啦
**填写时间**：{}
**提交时间**：{}

-----  

{}  

-----  
  
"#,
        start_time_human, end_time_human, q_and_a
    );
    md.to_string()
}
