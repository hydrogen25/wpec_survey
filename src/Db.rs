use std::{net::IpAddr, path::Path};

use anyhow::{Ok, Result};
use dashmap::DashSet;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::{
    Config::get_config,
    Sender::{fmt_survey, send_all},
    Structs::SurveyRequest,
};

pub async fn init() -> Result<()> {
    tokio::try_join!(init_csv(), init_json())?;
    Ok(())
}

pub async fn new_survey(
    client_ip: String,
    time_stamp: i64,
    time_human: String,
    data: SurveyRequest,
) -> Result<()> {
    let move_c = data.clone();
    let re = tokio::spawn(async move {
        tokio::try_join!(
            add_survey_to_csv(client_ip, time_stamp, time_human, move_c.clone()),
            add_survey_to_json(serde_json::to_value(move_c.clone()).unwrap()),
        )
        .unwrap()
    })
    .await;
    if let Err(e) = re {
        log::error!("{:#?}", e);
    }
    let msg = data;
    send_all(format!("{:#?}", fmt_survey(msg))).await;

    Ok(())
}

async fn add_survey_to_csv(
    client_ip: String,
    time_stamp: i64,
    time_human: String,
    data: SurveyRequest,
) -> Result<()> {
    let cfg = get_config();
    let posi: String = cfg.data_position.clone().unwrap_or("./".to_string());
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{posi}/data.csv"))
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
async fn init_csv() -> Result<()> {
    //如果不存在data.csv就自动建表
    let cfg = get_config();
    let posi: String = cfg.data_position.clone().unwrap_or("./".to_string());
    if !std::path::Path::new(&format!("{}/data.csv", posi)).exists() {
        std::fs::File::create(format!("{posi}/data.csv"))?;
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("{posi}/data.csv"))
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

async fn init_json() -> Result<()> {
    let cfg = get_config();

    let posi: String = cfg.data_position.clone().unwrap_or("./".to_string());
    if !std::path::Path::exists(std::path::Path::new(&format!("{posi}/data.json"))) {
        std::fs::File::create(format!("{posi}/data.json"))?;
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("{posi}/data.json"))
            .await?;
    }

    Ok(())
}

async fn add_survey_to_json(json: serde_json::Value) -> Result<()> {
    let cfg = get_config();
    let posi: String = cfg.data_position.clone().unwrap_or("./".to_string());
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{posi}/data.json"))
        .await?;

    file.write_all(format!("{},\n", json).as_bytes()).await?;
    file.flush().await?;
    Ok(())
}
