use crate::Config::get_config;
use crate::Db::add_survey_to_csv;
use crate::Structs::*;
use anyhow::Result;

use chrono_tz::Asia::Shanghai;
use salvo::prelude::*;

pub async fn start_api_server() -> Result<()> {
    let cfg = get_config();
    let acceptor = TcpListener::new(cfg.host.clone()).ttl(8964).bind().await;

    let router = Router::new();
    tokio::spawn(async move { Server::new(acceptor).serve(router).await });
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
        Ok(survey) => add_survey_to_csv(client_ip, time_stamp, time_human, survey).await,
        Err(_) => {
            res.render(Json(SubmitResponse::new(500, None)));
        }
    };
    res.render(Json(SubmitResponse::new(500, None)));
}
