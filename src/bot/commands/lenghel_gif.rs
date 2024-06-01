use serenity::all::CreateCommand;

pub const LENGHEL_GIF: &'static str = "lenghel-gif";

pub fn create() -> CreateCommand {
    CreateCommand::new("lenghel-gif").description("Un gif păstă medie")
}
