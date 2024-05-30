pub mod api;
pub mod bot;
pub mod model;
pub mod utils;

use std::env;

use bot::handler::Handler;
use dotenv::dotenv;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN expected");
    let _api_key = env::var("YT_API_KEY").expect("YT_API_KEY expected");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler::new())
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
