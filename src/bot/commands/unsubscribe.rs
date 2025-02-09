use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, Mentionable,
};
use crate::impl_deref_command_interaction;
use super::{respond::RespondToInteraction, slash_commands::Commands};
pub const UNSUBSCRIBE: &'static str = "unsubscribe";

pub struct UnSubscribeInteraction<'a>(pub &'a CommandInteraction);

impl_deref_command_interaction!(UnSubscribeInteraction<'a>);

pub fn create() -> CreateCommand {
    CreateCommand::new(UNSUBSCRIBE).description("Bagă un unsubscribe dacă ești un bivol")
}

impl<'a> RespondToInteraction<UnSubscribeInteraction<'a>> for Commands {
    async fn respond(
        &self,
        interaction: UnSubscribeInteraction<'a>,
        ctx: &Context,
    ) -> anyhow::Result<CreateInteractionResponse> {
        let users = self.subscriber_storage.all().await?;
        let content = if users.contains(&interaction.user.id) {
            self.subscriber_storage.remove(&interaction.user.id).await?;
            format!("Ești sub medie :(( {}", interaction.user.mention())
        } else {
            format!(
                "Bă, da tu nu ești subscribed bivolule {}",
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