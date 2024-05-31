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
        YoutubeChannel {
            name: "Imi Place Sa Mananc".to_string(),
            api,
        }
    }

    pub fn lenghel(api_key: String) -> Self {
        let api = YoutubeChannelApi::new(api_key, dotenv!("LENGHEL_UPLOADS_ID").to_string());
        YoutubeChannel {
            name: "Lenghel".to_string(),
            api,
        }
    }
}
