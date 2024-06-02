use anyhow::Result;
use serenity::{
    all::{
        Command, CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    futures::future::join_all,
};
use std::sync::Arc;

use crate::config::Config;

use super::{
    lenghel_gif::{self, LenghelGif, LENGHEL_GIF},
    respond::RespondToInteraction,
    subscribe::{self, Subscribe, SUBSCRIBE},
};

pub struct Commands {
    pub config: Arc<Config>,
    commands: Vec<CreateCommand>,
}

impl Commands {
    pub fn new(config: Arc<Config>) -> Self {
        Commands {
            config,
            commands: vec![lenghel_gif::create(), subscribe::create()],
        }
    }

    pub async fn register(&self, ctx: &Context) -> Result<Vec<Command>> {
        let commands = join_all(
            self.commands
                .iter()
                .map(|c| Command::create_global_command(&ctx.http, c.clone())),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

        Ok(commands)
    }

    pub async fn execute(&self, command: CommandInteraction, ctx: &Context) -> Result<()> {
        let res: CreateInteractionResponse = match command.data.name.as_str() {
            LENGHEL_GIF => self.respond(LenghelGif(&command), &ctx),
            SUBSCRIBE => self.respond(Subscribe(&command), &ctx),
            _ => unimplemented_command(),
        };

        command.create_response(&ctx.http, res).await?;

        Ok(())
    }
}

fn unimplemented_command() -> CreateInteractionResponse {
    let res_msg = CreateInteractionResponseMessage::new()
        .content("Deci effectiv nu se poate așa ceva".to_string());

    CreateInteractionResponse::Message(res_msg)
}
