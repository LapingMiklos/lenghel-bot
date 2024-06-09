use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use cron::Schedule;
use serenity::all::{Context, CreateMessage, EventHandler, Interaction, Ready};
use serenity::async_trait;
use shuttle_persist::PersistInstance;
use tokio::time;

use crate::config::Config;
use crate::db::subscriber_storage::SubscriberStorage;
use crate::db::threadsafe_storage::ThreadSafeStorage;
use crate::model::channel::YoutubeChannel;
use crate::utils::GetRandom;

use super::commands::slash_commands::Commands;
use crate::model::video::Video;
use crate::utils::discord::broadcast_message;
use crate::utils::messaging::{create_quote_message, create_video_message};

pub struct Handler {
    channels: Vec<YoutubeChannel>,
    config: Arc<Config>,
    _storage: Arc<ThreadSafeStorage>,
    subscriber_storage: SubscriberStorage,
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
        let subscriber_storage =
            SubscriberStorage::new(storage.clone()).expect("Unable to create storage");
        Handler {
            channels,
            config: config.clone(),
            _storage: storage.clone(),
            subscriber_storage: subscriber_storage.clone(),
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

        self.notification_thread(&ctx);
        self.daily_quote(&ctx);
    }
}

impl Handler {
    fn notification_thread(&self, ctx: &Context) {
        let ctx = ctx.clone();
        let channels = self.channels.clone();
        let subscriber_storage = self.subscriber_storage.clone();

        tokio::spawn(async move {
            let mut request_interval = time::interval(Duration::from_secs(120));
            let mut last_videos: HashMap<&YoutubeChannel, Video> = HashMap::new();

            loop {
                request_interval.tick().await;

                for channel in channels.iter() {
                    match send(channel, last_videos.get(channel), &subscriber_storage, &ctx).await {
                        Ok(Some(video)) => {
                            last_videos.insert(channel, video);
                        }
                        Err(err) => println!("Err: {err}"),
                        _ => {}
                    }
                }
            }
        });

        async fn send(
            channel: &YoutubeChannel,
            last_video: Option<&Video>,
            subscriber_storage: &SubscriberStorage,
            ctx: &Context,
        ) -> anyhow::Result<Option<Video>> {
            if let Some(video) = channel.api.get_recent_video().await? {
                let users = subscriber_storage.all().unwrap_or(HashSet::new());
                if !last_video.is_some_and(|v| v == &video) {
                    broadcast_message(ctx, create_video_message(&video, channel), Some(&users))
                        .await?;
                }
                Ok(Some(video))
            } else {
                Ok(None)
            }
        }
    }

    fn daily_quote(&self, ctx: &Context) {
        let ctx = ctx.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            let cron_expr = "0 * * * * * *";
            let schedule = Schedule::from_str(cron_expr).expect("Invalid cron expression");

            loop {
                let now = Utc::now();
                let next = schedule.upcoming(Utc).take(1).next().unwrap_or(now);
                let duration = (next - now).to_std().unwrap_or(Duration::from_secs(0));

                time::sleep(duration).await;

                let msg: CreateMessage = match config.quotes.get_random() {
                    Some(quote) => create_quote_message(quote, config.thumbnails.get_random()),
                    None => CreateMessage::new().content("Nu mai am citate :("),
                };

                if let Err(err) = broadcast_message(&ctx, msg, None).await {
                    println!("Err: {err}")
                }
            }
        });
    }
}
