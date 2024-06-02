pub mod discord;
pub mod messaging;

#[macro_export]
macro_rules! impl_deref_command_interaction {
    ($type:ty) => {
        use std::ops::Deref;
        impl<'a> Deref for $type {
            type Target = CommandInteraction;

            fn deref(&self) -> &Self::Target {
                self.0
            }
        }
    };
}
