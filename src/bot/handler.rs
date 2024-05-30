use std::time::Duration;

use serenity::all::{Context, CreateMessage, EventHandler, Ready};
use serenity::async_trait;
use serenity::model::channel::Message;
use tokio::time;

use crate::utils::discord_utils::broadcast_message;

pub struct Handler;

impl Handler {
    pub fn new() -> Handler {
        Handler
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "/lenghel" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Salut! Lenghel aicia").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        let ctx_clone = ctx.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(10));

            loop {
                interval.tick().await;

                if let Err(err) =
                    broadcast_message(&ctx_clone, CreateMessage::new().content("Salut!")).await
                {
                    println!("Err: {}", err);
                };
            }
        });
    }
}
