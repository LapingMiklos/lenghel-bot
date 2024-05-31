use reqwest::Error;
use serde::Deserialize;

use crate::model::video::Video;

#[derive(Clone, Debug)]
pub struct YoutubeChannelApi {
    api_key: String,
    channel_id: String,
}

impl YoutubeChannelApi {
    pub fn new(api_key: String, channel_id: String) -> Self {
        YoutubeChannelApi {
            api_key,
            channel_id,
        }
    }

    pub async fn get_recent_video(&self) -> Result<Option<Video>, Error> {
        #[derive(Deserialize, Debug)]
        struct ApiResponse {
            items: Vec<Video>,
        }

        let url = format!(
            "https://www.googleapis.com/youtube/v3/playlistItems?key={}&playlistId={}&part=snippet,id&maxResults=1",
            self.api_key, self.channel_id
        );

        let res: ApiResponse = reqwest::get(&url).await?.json().await?;

        Ok(res.items.into_iter().next().filter(|v| v.is_recent()))
    }
}

impl PartialEq for YoutubeChannelApi {
    fn eq(&self, other: &Self) -> bool {
        self.channel_id == other.channel_id
    }
}
