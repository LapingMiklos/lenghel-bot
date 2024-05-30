use dotenv_codegen::dotenv;
use reqwest::Error;
use serde::Deserialize;

use crate::model::video::Video;

pub struct YoutubeChannelApi {
    api_key: String,
    channel_id: &'static str,
}

impl YoutubeChannelApi {
    pub fn new_lenghel_api(api_key: String) -> Self {
        YoutubeChannelApi {
            api_key,
            channel_id: dotenv!("LENGHEL_CHANNEL_ID"),
        }
    }

    pub fn new_imi_place_api(api_key: String) -> Self {
        YoutubeChannelApi {
            api_key,
            channel_id: dotenv!("IMI_PLACE_CHANNEL_ID"),
        }
    }

    pub async fn get_last_video(&self) -> Result<Option<Video>, Error> {
        #[derive(Deserialize, Debug)]
        struct ApiResponse {
            items: Vec<Video>,
        }

        let url = format!(
            "https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet,id&order=date&maxResults=1",
            self.api_key, self.channel_id
        );

        let res: ApiResponse = reqwest::get(&url).await?.json().await?;

        Ok(res.items.into_iter().next())
    }
}
