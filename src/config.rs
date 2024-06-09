use serde::Deserialize;

use crate::model::Quote;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    pub gifs: Vec<String>,
    pub quotes: Vec<Quote>,
    pub ratings: Vec<String>,
    pub thumbnails: Vec<String>,
}
