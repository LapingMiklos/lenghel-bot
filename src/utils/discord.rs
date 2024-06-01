use anyhow::Result;

use serenity::all::{ChannelType, Context, CreateMessage, GuildChannel};

pub fn choose_channel(channels: Vec<GuildChannel>) -> Option<GuildChannel> {
    let mut channels = channels.into_iter();
    let first_channel = channels.next();

    channels
        .find(|c| c.name.contains("lenghel"))
        .or(first_channel)
}

pub async fn broadcast_message(ctx: &Context, msg: CreateMessage) -> Result<()> {
    let guilds = ctx.cache.guilds();

    for guild_id in guilds {
        let guild = ctx.http.get_guild(guild_id).await?;
        let mut channels: Vec<_> = guild
            .channels(&ctx.http)
            .await?
            .into_iter()
            .filter_map(|(_, c)| {
                if c.kind == ChannelType::Text {
                    Some(c)
                } else {
                    None
                }
            })
            .collect();
        channels.sort_by(|a, b| a.name.cmp(&b.name));

        if let Some(channel) = choose_channel(channels) {
            if let Err(err) = channel.id.send_message(&ctx.http, msg.clone()).await {
                println!("Err: {}", err)
            }
        }
    }

    Ok(())
}
