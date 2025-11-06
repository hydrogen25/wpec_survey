use crate::Config::get_config;
use crate::Db::new_survey;
use crate::Structs::*;
use anyhow::Result;
use chrono_tz::Asia::Shanghai;
use salvo::cors::Cors;
use salvo::http::Method;
use salvo::prelude::*;
use salvo_extra::concurrency_limiter::max_concurrency;
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

    let router = Router::with_path("/survey")
        .hoop(max_concurrency(50))
        .post(submit);
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
    match req.parse_json::<SurveyRequest>().await {
        Ok(survey) => match new_survey(client_ip, time_stamp, time_human, survey.clone()).await {
            Ok(_) => {
                if !check(time_stamp, survey) {
                    res.render(Json(SubmitResponse::new(
                        400,
                        Some("问卷不合法".to_string()),
                    )));
                    return;
                }
                res.render(Json(SubmitResponse::new(200, None)));
            }
            Err(e) => {
                res.render(Json(SubmitResponse::new(500, Some(e.to_string()))));
            }
        },
        Err(e) => {
            res.render(Json(SubmitResponse::new(400, Some(e.to_string()))));
        }
    }
}

//返回是否合法
//true->问卷无误
//false->问卷有问题
pub fn check(time_stamp: i64, survey: SurveyRequest) -> bool {
    let cfg_pro_nums = get_config().problems.iter().count();
    let crt_pro_nums = survey.data.iter().count();
    if cfg_pro_nums != crt_pro_nums {
        dbg!(cfg_pro_nums, crt_pro_nums);
        return false;
    }

    let submit_time = time_stamp as u64;
    let start_time = survey.start_time;

    if submit_time < start_time {
        dbg!("20");

        return false;
    } else if submit_time - start_time < 10 {
        dbg!("21");

        return false;
    }

    for spro in &survey.data {
        for cpro in &get_config().problems {
            if spro.id == cpro.id {
                if spro.is_required != cpro.is_required {
                    dbg!("23");

                    return false;
                }
            }
        }

        match spro.is_required {
            true => {
                if spro.answer.is_none() {
                    dbg!("24");

                    return false;
                }
            }
            false => {
                dbg!("25");
            }
        }
    }

    true
}
