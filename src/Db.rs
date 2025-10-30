use anyhow::Result;
use tokio::fs::OpenOptions;

use crate::Structs::SurveyRequest;

pub async fn add_survey_to_csv(
    client_ip: String,
    time_stamp: i64,
    time_human: String,
    survey: SurveyRequest,
) -> Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("data.csv")
        .await
        .expect("打开 data.csv 失败");
    let mut writer = csv_async::AsyncWriter::from_writer(file);
    writer.write_record(&["gunmu", "gunmu"]).await?;
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

        writer.write_record(&["hello", "world"]).await?;

        writer.flush().await?;
    }
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("data.csv")
        .await
        .expect("打开 data.csv 失败");
    let mut writer = csv_async::AsyncWriter::from_writer(file);
    writer.write_record(&["gunmu", "gunmu"]).await?;
    writer.flush().await?;
    Ok(())
}
