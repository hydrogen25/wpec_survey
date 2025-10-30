use anyhow::Ok;
use server::ApiServer::start_api_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    start_api_server().await?;

    Ok(())
}
