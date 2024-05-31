use std::hash::Hash;

use dotenv_codegen::dotenv;

use crate::api::youtube::YoutubeChannelApi;

#[derive(Clone, Debug)]
pub struct YoutubeChannel {
    pub name: String,
    pub api: YoutubeChannelApi,
}

impl YoutubeChannel {
    pub fn imi_place_sa_mananc(api_key: String) -> Self {
        let api = YoutubeChannelApi::new(api_key, dotenv!("IMI_PLACE_UPLOADS_ID").to_string());
        println!("Imi place Sa Mananc created: {}", api.channel_id);
        YoutubeChannel {
            name: "Imi Place Sa Mananc".to_string(),
            api,
        }
    }

    pub fn lenghel(api_key: String) -> Self {
        let api = YoutubeChannelApi::new(api_key, dotenv!("LENGHEL_UPLOADS_ID").to_string());
        println!("Lenghel created: {}", api.channel_id);
        YoutubeChannel {
            name: "Lenghel".to_string(),
            api,
        }
    }
}

impl PartialEq for YoutubeChannel {
    fn eq(&self, other: &Self) -> bool {
        self.api.channel_id == other.api.channel_id
    }
}

impl Eq for YoutubeChannel {}

impl Hash for YoutubeChannel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
