use serenity::all::{Context, CreateInteractionResponse};

pub trait RespondToInteraction<T> {
    fn respond(&self, interaction: T, ctx: &Context) -> CreateInteractionResponse;
}
