use anyhow::Result;
use tokio::fs::OpenOptions;

use crate::{Config::get_config, Structs::SurveyRequest};

pub async fn add_survey_to_csv(
    client_ip: String,
    time_stamp: i64,
    time_human: String,
    data: SurveyRequest,
) -> Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("data.csv")
        .await
        .expect("打开 data.csv 失败");
    let mut writer = csv_async::AsyncWriter::from_writer(file);

    let mut re = data
        .clone()
        .data
        .into_iter()
        .map(|a| a.answer.unwrap_or("空".to_string()))
        .collect::<Vec<String>>();

    re.push(data.start_time.to_string());
    re.push(time_stamp.to_string());
    re.push(time_human.to_string());
    re.push(client_ip.to_string());
    writer.write_record(re).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn init_csv() -> Result<()> {
    //如果不存在data.csv就自动建表
    if !std::path::Path::new("data.csv").exists() {
        std::fs::File::create("data.csv")?;
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("data.csv")
            .await?;
        let mut writer = csv_async::AsyncWriter::from_writer(file);
        let cfg = get_config();

        let headers = cfg
            .problems
            .iter()
            .map(|p| p.question.clone())
            .chain(
                vec![
                    "起始时间戳",
                    "结束时间戳",
                    "结束时间（人类可读）",
                    "IP",
                    "是否为AI生成",
                    "备注",
                ]
                .into_iter()
                .map(ToString::to_string),
            )
            .collect::<Vec<_>>();

        writer.write_record(headers).await?;

        writer.flush().await?;
    }

    Ok(())
}
