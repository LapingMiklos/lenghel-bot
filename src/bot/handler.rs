use std::error::Error;
use std::time::Duration;

use serenity::all::{Context, CreateMessage, EventHandler, Ready};
use serenity::async_trait;
use serenity::futures::future::join_all;
use serenity::model::channel::Message;
use tokio::time;

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
            let mut interval = time::interval(Duration::from_secs(1000));

            loop {
                interval.tick().await;

                if let Err(err) = do_the_thing(&ctx_clone).await {
                    println!("Err: {}", err);
                };
            }
        });
    }
}

async fn do_the_thing(ctx: &Context) -> Result<(), Box<dyn Error>> {
    let guilds = ctx.cache.guilds();

    for guild_id in guilds {
        let guild = ctx.http.get_guild(guild_id).await?;
        let channels = guild.channels(&ctx.http).await?;

        join_all(
            channels
                .iter()
                .filter_map(|(channel_id, channel)| {
                    if channel.is_text_based() {
                        Some(channel_id)
                    } else {
                        None
                    }
                })
                .map(|channel_id| {
                    channel_id.send_message(
                        &ctx.http,
                        CreateMessage::new().content(
                            "https://tenor.com/view/lenghel-crying-gif-12267950906633408140",
                        ),
                    )
                }),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    }
    Ok(())
}
