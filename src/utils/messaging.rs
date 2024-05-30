use serenity::all::CreateMessage;

use crate::model::video::Video;

pub fn create_video_message(video: &Video) -> CreateMessage {
    CreateMessage::new().content(format!(
        "Salut lenghel aicia! Am postat un nou clip: https://www.youtube.com/watch?v={}",
        video.id.video_id
    ))
}
