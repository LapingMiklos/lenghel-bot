use std::sync::Arc;

use serenity::{
    all::{Command, CommandInteraction, Context, CreateCommand},
    futures::future::join_all,
};

use crate::config::Config;

use super::lenghel_gif::{self};

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

    pub async fn register(&self, ctx: &Context) -> serenity::Result<Vec<Command>> {
        join_all(
            self.commands
                .iter()
                .map(|c| Command::create_global_command(&ctx.http, c.clone())),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
    }

    pub async fn execute(command: CommandInteraction) {}
}
