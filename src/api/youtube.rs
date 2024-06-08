use anyhow::Result;

use serde::Deserialize;

use crate::model::video::Video;

#[derive(Clone, Debug)]
pub struct YoutubeChannelApi {
    api_key: String,
    pub channel_id: String,
}

impl YoutubeChannelApi {
    pub fn new(api_key: impl Into<String>, channel_id: impl Into<String>) -> Self {
        YoutubeChannelApi {
            api_key: api_key.into(),
            channel_id: channel_id.into(),
        }
    }

    pub async fn get_recent_video(&self) -> Result<Option<Video>> {
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

impl Eq for YoutubeChannelApi {}
