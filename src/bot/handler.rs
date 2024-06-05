use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use serenity::all::{Context, EventHandler, Interaction, Ready};
use serenity::async_trait;
use shuttle_persist::PersistInstance;
use tokio::time;

use crate::config::Config;
use crate::db::subscriber_storage::SubscriberStorage;
use crate::db::threadsafe_storage::ThreadSafeStorage;
use crate::model::channel::YoutubeChannel;

use super::commands::slash_commands::Commands;
use crate::model::video::Video;
use crate::utils::discord::broadcast_message;
use crate::utils::messaging::create_video_message;

pub struct Handler {
    channels: Vec<YoutubeChannel>,
    _config: Arc<Config>,
    _storage: Arc<ThreadSafeStorage>,
    _subscriber_storage: SubscriberStorage,
    commands: Commands,
}

impl Handler {
    pub fn new(
        channels: Vec<YoutubeChannel>,
        config: Config,
        persist_instance: PersistInstance,
    ) -> Handler {
        let config = Arc::new(config);
        let storage = Arc::new(ThreadSafeStorage::new(persist_instance));
        let subscriber_storage = SubscriberStorage::new(storage.clone());
        Handler {
            channels,
            _config: config.clone(),
            _storage: storage.clone(),
            _subscriber_storage: subscriber_storage.clone(),
            commands: Commands::new(config, subscriber_storage),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            if let Err(err) = self.commands.execute(command, &ctx).await {
                println!("Cannot respond to slash command: {err}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot started: {}", ready.user.name);

        match self.commands.register(&ctx).await {
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
