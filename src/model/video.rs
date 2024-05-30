use chrono::{DateTime, Duration, Utc};
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

impl Video {
    pub fn is_recent(&self) -> bool {
        if let Ok(date) =
            DateTime::parse_from_rfc3339(&self.snippet.published_at).map(|d| d.with_timezone(&Utc))
        {
            let now = Utc::now();
            let duration = if now > date { now - date } else { date - now };

            duration <= Duration::days(15)
        } else {
            false
        }
    }
}
