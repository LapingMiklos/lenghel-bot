use super::{respond::RespondToInteraction, slash_commands::Commands};
use crate::impl_deref_command_interaction;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, Mentionable,
};
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
        let users = self.subscriber_storage.all().await?;
        let content = if users.contains(&interaction.user.id) {
            format!(
                "Bă, da tu ai mai dat subscribe bivolule {}",
                interaction.user.mention()
            )
        } else {
            self.subscriber_storage.add(&interaction.user.id).await?;
            format!(
                "Mersi pentru subscribe, ești păstă medie {}",
                interaction.user.mention()
            )
        };
        interaction
            .user
            .direct_message(&ctx.http, CreateMessage::new().content(&content))
            .await?;
        let res_msg = CreateInteractionResponseMessage::new().content(&content);
        Ok(CreateInteractionResponse::Message(res_msg))
    }
}
