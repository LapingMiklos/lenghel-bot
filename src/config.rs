use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    pub gifs: Vec<String>,
    pub quotes: Vec<String>,
}
