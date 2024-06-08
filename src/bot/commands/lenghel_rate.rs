use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage, Mentionable,
};

use crate::{impl_deref_command_interaction, utils::GetRandom};

use super::{respond::RespondToInteraction, slash_commands::Commands};

pub const LENGHEL_RATE: &'static str = "lenghel-rate";
const USER: &'static str = "user";

pub struct LenghelRateInteraction<'a>(pub &'a CommandInteraction);
impl_deref_command_interaction!(LenghelRateInteraction<'a>);

pub fn create() -> CreateCommand {
    let option =
        CreateCommandOption::new(CommandOptionType::User, USER, "the user to interact with")
            .required(true);
    CreateCommand::new(LENGHEL_RATE)
        .description("Un rating pentru prieteni")
        .add_option(option)
}

impl<'a> RespondToInteraction<LenghelRateInteraction<'a>> for Commands {
    async fn respond(
        &self,
        interaction: LenghelRateInteraction<'a>,
        _: &Context,
    ) -> anyhow::Result<CreateInteractionResponse> {
        let content: String = if let Some(CommandDataOptionValue::User(uid)) = interaction
            .data
            .options
            .iter()
            .filter(|o| o.name == USER)
            .next()
            .map(|o| &o.value)
        {
            let rating: &str = match self.config.ratings.get_random() {
                Some(rating) => rating,
                None => "Nu știu ce să zic",
            };
            format!("{} {}", rating, uid.mention())
        } else {
            "Something went wrong".into()
        };
        let res_msg = CreateInteractionResponseMessage::new().content(content);
        Ok(CreateInteractionResponse::Message(res_msg))
    }
}
