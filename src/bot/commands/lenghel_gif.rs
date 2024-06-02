use std::ops::Deref;

use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use super::{respond::RespondToInteraction, slash_commands::Commands};

pub const LENGHEL_GIF: &'static str = "lenghel-gif";

pub struct LenghelGif<'a>(pub &'a CommandInteraction);

impl<'a> Deref for LenghelGif<'a> {
    type Target = CommandInteraction;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

pub fn create() -> CreateCommand {
    CreateCommand::new(LENGHEL_GIF).description("Un gif păstă medie")
}

impl RespondToInteraction<LenghelGif<'_>> for Commands {
    fn respond(&self, _: LenghelGif, _: &Context) -> CreateInteractionResponse {
        let res_msg = CreateInteractionResponseMessage::new().content(self.config.gifs.get());
        CreateInteractionResponse::Message(res_msg)
    }
}
