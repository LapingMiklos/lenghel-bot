use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, Mentionable,
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

impl<'a> RespondToInteraction<SubscribeInteraction<'a>> for Commands {
    async fn respond(
        &self,
        interaction: SubscribeInteraction<'a>,
        ctx: &Context,
    ) -> anyhow::Result<CreateInteractionResponse> {
        let content = format!(
            "Mersi pentru subscribe, ești păstă medie {}",
            interaction.user.mention()
        );

        interaction
            .user
            .direct_message(&ctx.http, CreateMessage::new().content(&content))
            .await?;

        let res_msg = CreateInteractionResponseMessage::new().content(&content);
        Ok(CreateInteractionResponse::Message(res_msg))
    }
}
