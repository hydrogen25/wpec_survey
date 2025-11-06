use crate::Config::get_config;
use crate::Db::new_survey;
use crate::Structs::*;
use anyhow::Result;
use salvo::cors::Cors;
use salvo::http::Method;
use std::sync::Arc;

use chrono_tz::Asia::Shanghai;
use salvo::prelude::*;
use tokio::signal;

pub async fn start_api_server() -> Result<()> {
    let cfg = get_config();
    let host = cfg.host.clone();
    let acceptor = TcpListener::new(host).ttl(222).bind().await;
    let cors = Cors::new()
        .allow_origin(cfg.api_address.as_str())
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .allow_headers("authorization")
        .into_handler();

    let router = Router::with_path("/survey").post(submit);
    let service = Service::new(router).hoop(cors);
    tokio::spawn(async move {
        Server::new(acceptor).serve(service).await;
    });
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
    //dbg!(req.parse_json::<SurveyRequest>().await.unwrap());
    match req.parse_json::<SurveyRequest>().await {
        Ok(survey) => match new_survey(client_ip, time_stamp, time_human, survey).await {
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
