use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use serenity::all::{Context, EventHandler, Ready};
use serenity::async_trait;
use serenity::model::channel::Message;
use tokio::time;

use crate::config::Config;
use crate::model::channel::YoutubeChannel;

use crate::model::video::Video;
use crate::utils::discord::broadcast_message;
use crate::utils::messaging::create_video_message;

pub struct Handler {
    channels: Vec<YoutubeChannel>,
    config: Arc<Config>,
}

impl Handler {
    pub fn new(channels: Vec<YoutubeChannel>, config: Arc<Config>) -> Handler {
        Handler { channels, config }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "/lenghel-gif" {
            if let Err(why) = msg.channel_id.say(&ctx.http, self.config.gifs.get()).await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        let ctx_clone = ctx.clone();
        let channels = self.channels.clone();

        tokio::spawn(async move {
            let mut request_interval = time::interval(Duration::from_secs(120));
            let mut last_video: HashMap<&YoutubeChannel, Video> = HashMap::new();

            loop {
                request_interval.tick().await;

                for channel in channels.iter() {
                    match channel.api.get_recent_video().await {
                        Ok(Some(video)) => {
                            if !last_video.get(&channel).is_some_and(|v| v == &video) {
                                if let Err(err) = broadcast_message(
                                    &ctx_clone,
                                    create_video_message(&video, channel),
                                )
                                .await
                                {
                                    println!("Err: {}", err);
                                };
                                last_video.insert(&channel, video);
                            }
                        }
                        Ok(None) => {}
                        Err(err) => {
                            println!("Err: {}", err);
                        }
                    }
                }
            }
        });
    }
}
