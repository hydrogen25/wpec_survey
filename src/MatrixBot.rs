use anyhow::Ok;
use matrix_sdk::{
    Client, Room, RoomState,
    config::SyncSettings,
    ruma::{
        api::client::sync::sync_events::v3::JoinedRoom,
        events::room::message::{
            MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent,
        },
    },
    sync::SyncResponse,
};

use crate::Sender::BotSender;

pub struct MatrixBot {
    homeserver_url: String,
    username: String,
    password: String,
    pub client: Client,
    pub response: SyncResponse,
    pub settings: SyncSettings,
}

impl MatrixBot {
    pub async fn new(homeserver_url: String, username: String, password: String) -> Self {
        let client = Client::builder()
            .homeserver_url(homeserver_url.clone())
            .build()
            .await
            .unwrap();
        client
            .matrix_auth()
            .login_username(&username, &password)
            .initial_device_display_name("command bot")
            .await
            .expect("初始化机器人失败，请检查网络环境与配置文件");
        dbg!("已作为 {} 登陆", username.clone());
        let response = client.sync_once(SyncSettings::default()).await.unwrap();
        dbg!("test1S");
        let settings = SyncSettings::default().token(response.clone().next_batch);
        client.add_event_handler(
            |event: OriginalSyncRoomMessageEvent, room: Room| async move {
                println!("on_room_message entered");
                // 只处理已加入的房间

                if let MessageType::Text(text_content) = &event.content.msgtype {
                    println!("收到消息: {}", text_content.body);
                    if text_content.body.contains("!party") {
                        let content =
                            RoomMessageEventContent::text_plain("🎉🎊🥳 let's PARTY!! 🥳🎊🎉");
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

        client
            .sync(settings.clone())
            .await
            .expect("错误！机器人同步出错");
        return Self {
            homeserver_url,
            username,
            password,
            client: client.clone(),
            response,
            settings,
        };
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
        let content = RoomMessageEventContent::text_plain("🎉🎊🥳 let's PARTY!! 🥳🎊🎉");

        println!("sending");

        // send our message to the room we found the "!party" command in
        room.send(content).await.unwrap();

        println!("message sent");
    }
}

impl BotSender for MatrixBot {
    async fn send(&self, message: String) -> anyhow::Result<()> {
        Ok(())
    }
}
