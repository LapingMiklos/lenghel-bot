use serenity::all::{Context, CreateInteractionResponse};

pub trait RespondToInteraction<T> {
    async fn respond(
        &self,
        interaction: T,
        ctx: &Context,
    ) -> anyhow::Result<CreateInteractionResponse>;
}
