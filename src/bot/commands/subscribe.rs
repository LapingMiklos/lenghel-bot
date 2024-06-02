use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use crate::impl_deref_command_interaction;

use super::{respond::RespondToInteraction, slash_commands::Commands};

pub const SUBSCRIBE: &'static str = "subscribe";
pub struct SubscribeInteraction<'a>(pub &'a CommandInteraction);
impl_deref_command_interaction!(SubscribeInteraction<'a>);

pub fn create() -> CreateCommand {
    CreateCommand::new(SUBSCRIBE)
        .description("Bagă un subscribe și îți trimit notificații în DM-uri")
}

impl RespondToInteraction<SubscribeInteraction<'_>> for Commands {
    fn respond(&self, _command: SubscribeInteraction, _ctx: &Context) -> CreateInteractionResponse {
        let res_msg = CreateInteractionResponseMessage::new().content("Mersi fă");
        CreateInteractionResponse::Message(res_msg)
    }
}
