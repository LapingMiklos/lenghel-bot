use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use serenity::all::{
    Command, Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler,
    Interaction, Ready,
};
use serenity::async_trait;
use tokio::time;

use crate::config::Config;
use crate::model::channel::YoutubeChannel;

use super::commands::lenghel_gif;
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
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "lenghel-gif" => Some(self.config.gifs.get()),
                _ => Some("Deci effectiv nu se poate așa ceva".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot started: {}", ready.user.name);

        match Command::create_global_command(&ctx.http, lenghel_gif::create()).await {
            Ok(_) => {
                println!("Registered slash commands")
            }
            Err(err) => {
                println!("Failed to register slash command: {}", err)
            }
        }

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
