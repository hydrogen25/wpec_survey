pub trait BotSender {
    fn send(&self, message: String)
    -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}

async fn send_all<T: BotSender>(sender: &T, msg: String) {}
