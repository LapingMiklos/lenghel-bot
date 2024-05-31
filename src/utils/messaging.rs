use serenity::all::CreateMessage;

use crate::model::{channel::YoutubeChannel, video::Video};

pub fn create_video_message(video: &Video, channel: &YoutubeChannel) -> CreateMessage {
    CreateMessage::new().content(format!(
        "Salut {} aicia! Am postat un nou clip: https://www.youtube.com/watch?v={}",
        channel.name, video.snippet.resource_id.video_id
    ))
}
