use anyhow::Ok;
use server::{
    ApiServer::start_api_server, Config::get_config, Db::init, TgBot, bots::Bots::init_bots,
};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    init().await?;
    init_bots().await;
    start_api_server().await?;

    let cfg = get_config();
    println!("{:#?}", cfg);
    Ok(())
}
