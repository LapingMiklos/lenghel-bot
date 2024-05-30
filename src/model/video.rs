use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Video {
    pub id: VideoId,
    pub snippet: Snippet,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct VideoId {
    #[serde(rename = "videoId")]
    pub video_id: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Snippet {
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    pub title: String,
}
