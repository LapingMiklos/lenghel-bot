use serde::Deserialize;

use crate::model::gifs::Gifs;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    pub gifs: Gifs,
}
