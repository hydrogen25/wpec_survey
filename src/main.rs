use anyhow::Ok;
use server::{ApiServer::start_api_server, Config::get_config, Db::init};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init().await?;
    start_api_server().await?;

    let cfg = get_config();
    println!("{:#?}", cfg);
    Ok(())
}
