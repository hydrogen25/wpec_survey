use crate::Config::get_config;
use crate::Db::add_survey_to_csv;
use crate::Structs::*;
use anyhow::Result;

use chrono_tz::Asia::Shanghai;
use salvo::prelude::*;
use tokio::signal;

pub async fn start_api_server() -> Result<()> {
    // 只需要 host 字段，提前克隆并丢弃 cfg，避免其在异步任务/handler 中被捕获导致非 'static 生命周期问题
    let cfg = get_config();
    let host = cfg.host.clone();
    let acceptor = TcpListener::new(host).ttl(222).bind().await;

    let router = Router::with_path("/survey").post(submit);
    tokio::spawn(async move { Server::new(acceptor).serve(router).await });
    signal::ctrl_c().await.unwrap();
    println!("收到主线程关闭信号，任务结束");
    Ok(())
}

#[handler]
async fn submit(req: &mut Request, res: &mut Response) {
    let ip_from_xff = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next().map(|s| s.trim().to_string()));

    let ip_from_xreal = req
        .headers()
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim().to_string());

    let ip_from_remote = req.remote_addr().to_owned().to_string();

    let client_ip = ip_from_xff.or(ip_from_xreal).unwrap_or(ip_from_remote);
    let time_stamp = chrono::Local::now().timestamp();
    let time_human = chrono::Utc::now().with_timezone(&Shanghai).to_string();
    match req.parse_json::<SurveyRequest>().await {
        Ok(survey) => match add_survey_to_csv(client_ip, time_stamp, time_human, survey).await {
            Ok(_) => {
                res.render(Json(SubmitResponse::new(200, None)));
            }
            Err(_) => {
                res.render(Json(SubmitResponse::new(500, None)));
            }
        },
        Err(_) => {
            res.render(Json(SubmitResponse::new(400, None)));
        }
    }
}
