use anyhow::Ok;
use matrix_sdk::{
    Client, Room, RoomState,
    config::SyncSettings,
    ruma::events::room::message::{
        MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent,
    },
};
use std::time::Duration;
use tokio::sync::OnceCell;
use tokio_retry::{
    Retry,
    strategy::{ExponentialBackoff, jitter},
};

use crate::{Config::get_config, Sender::BotSender, bots::Bots::BotEnum};

pub static CLIENT: OnceCell<Client> = OnceCell::const_new();

pub async fn get_client() -> &'static Client {
    CLIENT
        .get_or_init(|| async {
            let config = get_config().clone().matrix_bot.unwrap();
            let homeserver_url = config.homeserver_url.clone();

            let client = Client::builder()
                .homeserver_url(homeserver_url)
                .build()
                .await
                .unwrap();
            log::info!("登陆中");
            client
                .matrix_auth()
                .login_username(&config.username, &config.password)
                .initial_device_display_name("command bot")
                .await
                .expect("登陆失败");
            log::info!("登陆完毕");

            let strategy = ExponentialBackoff::from_millis(10)
                .max_delay(Duration::from_secs(10))
                .map(jitter);

            let response = Retry::spawn(strategy, || async {
                client.sync_once(SyncSettings::default()).await
            })
            .await
            .unwrap();
            for r in client.joined_rooms().iter() {
                r.send(RoomMessageEventContent::text_markdown("🎉机器人已启动"))
                    .await
                    .unwrap();
            }

            let settings = SyncSettings::default().token(response.next_batch);
            let new_client = client.clone();
            tokio::spawn(async move {
                client.sync(settings).await.unwrap();
            });

            new_client
        })
        .await
}

#[derive(Debug, Clone)]
pub struct MatrixBot {}

impl MatrixBot {
    pub async fn new() -> Self {
        dbg!("1");
        let client = get_client().await;
        dbg!("10");
        client.add_event_handler(
            |event: OriginalSyncRoomMessageEvent, room: Room| async move {
                println!("on_room_message entered");
                // 只处理已加入的房间
                dbg!(event.clone());

                if let MessageType::Text(text_content) = &event.content.msgtype {
                    println!("收到消息: {}", text_content.body);
                    if text_content.body.contains("!ping") {
                        let content =
                            RoomMessageEventContent::text_markdown("🎉🎊🥳 let's PARTY!! 🥳🎊🎉");
                        println!("sending");
                        // 使用 JoinedRoom::send(content, None)
                        if let Err(e) = room.send(content).await {
                            eprintln!("send error: {:?}", e);
                        } else {
                            println!("message sent");
                        }
                    }
                } else {
                    println!("room not joined, skipping");
                }
            },
        );

        BotEnum::push_bot(BotEnum::MatrixBot(Self {})).await;

        return Self {};
    }
}

async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
    dbg!("test");

    if room.state() != RoomState::Joined {
        return;
    }
    let MessageType::Text(text_content) = event.content.msgtype else {
        return;
    };
    dbg!("{}", text_content.body.clone());
    if text_content.body.contains("!party") {
        let content = RoomMessageEventContent::text_markdown("🎉🎊🥳 let's PARTY!! 🥳🎊🎉");

        println!("sending");

        // send our message to the room we found the "!party" command in
        room.send(content).await.unwrap();

        println!("message sent");
    }
}

impl BotSender for MatrixBot {
    async fn send(&self, message: String) -> anyhow::Result<()> {
        let client = get_client().await;
        let content = RoomMessageEventContent::text_markdown(message);
        for room in client.joined_rooms() {
            room.send(content.clone()).await?;
        }
        Ok(())
    }
}
