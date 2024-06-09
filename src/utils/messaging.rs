use serenity::all::{CreateEmbed, CreateMessage};

use crate::model::{channel::YoutubeChannel, video::Video, Quote};

const THUMBNAIL: &'static str =
    "https://yt3.googleusercontent.com/ytc/AIdro_lrb0nnxYOmvzBP-5orDugrG8_X5Ylsjn5mtGkWftTOnA=s176-c-k-c0x00ffffff-no-rj";

pub fn create_video_message(video: &Video, channel: &YoutubeChannel) -> CreateMessage {
    CreateMessage::new().content(format!(
        "Salut {} aicia! Am postat un nou clip: https://www.youtube.com/watch?v={}",
        channel.name, video.snippet.resource_id.video_id
    ))
}

pub fn create_quote_message(quote: &Quote, thumbnail: Option<&str>) -> CreateMessage {
    let embed = CreateEmbed::new()
        .title("Daily Ionuț Lenghel quote:")
        .field("", format!("\"{}\"", quote.text), true)
        .thumbnail(thumbnail.unwrap_or(THUMBNAIL))
        .url(&quote.source);

    CreateMessage::new().add_embed(embed)
}
