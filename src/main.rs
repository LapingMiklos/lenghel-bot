pub mod api;
pub mod bot;
pub mod config;
pub mod model;
pub mod utils;
pub mod db;

use std::{fs::File, io::BufReader};

use bot::handler::Handler;
use config::Config;
use db::SubscriberStorage;
use dotenv::dotenv;
use model::channel::YoutubeChannel;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
    #[shuttle_shared_db::Postgres] db: shuttle_shared_db::SerdeJsonOperator,
) -> shuttle_serenity::ShuttleSerenity {
    dotenv().ok();

    let file = File::open("config.json").expect("config.json expected");
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).expect("JSON was not well-formatted");

    let token = secrets
        .get("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN expected");
    let yt_api_key = secrets.get("YT_API_KEY").expect("YT_API_KEY expected");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    if let Err(err) = SubscriberStorage::init(&db).await {
        panic!("Unable to write to db {}", err);
    }

    let client = Client::builder(&token, intents)
        .event_handler(Handler::new(
            vec![
                YoutubeChannel::lenghel(&yt_api_key),
                YoutubeChannel::imi_place_sa_mananc(&yt_api_key),
            ],
            config,
            db,
        ))
        .await
        .expect("Err creating client");

    Ok(client.into())
}
