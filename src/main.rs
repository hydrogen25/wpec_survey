use anyhow::Ok;
use server::{ApiServer::start_api_server, Config::get_config, Db::init_csv};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //start_api_server().await?;
    //init_csv().await?;
    //start_api_server().await?;
    let cfg = get_config();
    println!("{:#?}", cfg);
    Ok(())
}
