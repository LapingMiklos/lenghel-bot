use serde::{Deserialize, Serialize};

pub mod channel;
pub mod video;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Quote {
    pub text: String,
    pub source: String,
}
