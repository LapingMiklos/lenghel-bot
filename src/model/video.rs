use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Video {
    pub snippet: Snippet,
}

impl PartialEq for Video {
    fn eq(&self, other: &Self) -> bool {
        self.snippet.resource_id.video_id == other.snippet.resource_id.video_id
    }
}

#[derive(Deserialize, Debug)]
pub struct VideoId {
    #[serde(rename = "videoId")]
    pub video_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Snippet {
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    #[serde(rename = "resourceId")]
    pub resource_id: VideoId,
    pub title: String,
}

impl Video {
    pub fn is_recent(&self) -> bool {
        if let Ok(date) =
            DateTime::parse_from_rfc3339(&self.snippet.published_at).map(|d| d.with_timezone(&Utc))
        {
            let now = Utc::now();
            let duration = if now > date { now - date } else { date - now };

            duration <= Duration::minutes(10)
        } else {
            false
        }
    }
}
