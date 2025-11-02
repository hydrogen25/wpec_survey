use tokio::test;

use crate::MatrixBot::MatrixBot;

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
    let bot = MatrixBot::new(
        "https://matrix.catgirl.cloud".to_string(),
        "wpecsurveybot".to_string(),
        "mlmzy19171949".to_string(),
    )
    .await;
    //tokio::signal::ctrl_c().await.unwrap();
}
