use anyhow::Ok;
use server::{ApiServer::start_api_server, Db::init_csv};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //start_api_server().await?;
    init_csv().await?;
    start_api_server().await?;
    Ok(())
}
