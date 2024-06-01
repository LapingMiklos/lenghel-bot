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

use super::lenghel_gif::{self, LENGHEL_GIF};

pub struct Commands {
    config: Arc<Config>,
    commands: Vec<CreateCommand>,
}

impl Commands {
    pub fn new(config: Arc<Config>) -> Self {
        Commands {
            config,
            commands: vec![lenghel_gif::create()],
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
        let content: String = match command.data.name.as_str() {
            LENGHEL_GIF => self.config.gifs.get(),
            _ => "Deci effectiv nu se poate așa ceva".to_string(),
        };

        let data = CreateInteractionResponseMessage::new().content(content);
        let builder = CreateInteractionResponse::Message(data);
        command.create_response(&ctx.http, builder).await?;

        Ok(())
    }
}
