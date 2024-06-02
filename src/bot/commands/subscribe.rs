use std::ops::Deref;

use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use super::{respond::RespondToInteraction, slash_commands::Commands};

pub const SUBSCRIBE: &'static str = "subscribe";
pub struct Subscribe<'a>(pub &'a CommandInteraction);

impl<'a> Deref for Subscribe<'a> {
    type Target = CommandInteraction;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

pub fn create() -> CreateCommand {
    CreateCommand::new(SUBSCRIBE)
        .description("Bagă un subscribe și îți trimit notificații în DM-uri")
}

impl RespondToInteraction<Subscribe<'_>> for Commands {
    fn respond(&self, _command: Subscribe, _ctx: &Context) -> CreateInteractionResponse {
        let res_msg = CreateInteractionResponseMessage::new().content("Mersi fă");
        CreateInteractionResponse::Message(res_msg)
    }
}
