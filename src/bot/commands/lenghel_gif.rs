use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use crate::impl_deref_command_interaction;

use super::{respond::RespondToInteraction, slash_commands::Commands};

pub const LENGHEL_GIF: &'static str = "lenghel-gif";

pub struct LenghelGifInteraction<'a>(pub &'a CommandInteraction);
impl_deref_command_interaction!(LenghelGifInteraction<'a>);

pub fn create() -> CreateCommand {
    CreateCommand::new(LENGHEL_GIF).description("Un gif păstă medie")
}

impl<'a> RespondToInteraction<LenghelGifInteraction<'a>> for Commands {
    async fn respond(
        &self,
        _: LenghelGifInteraction<'a>,
        _: &Context,
    ) -> anyhow::Result<CreateInteractionResponse> {
        let res_msg = CreateInteractionResponseMessage::new().content(self.config.gifs.get());
        Ok(CreateInteractionResponse::Message(res_msg))
    }
}
