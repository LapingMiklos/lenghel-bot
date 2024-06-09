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

pub trait GetRandom<'a, T: ?Sized> {
    fn get_random(&'a self) -> Option<&'a T>;
}

impl<'a> GetRandom<'a, str> for Vec<String> {
    fn get_random(&'a self) -> Option<&'a str> {
        if self.len() == 0 {
            return None;
        }
        let i = rand::random::<usize>() % self.len();

        Some(&self[i])
    }
}

impl<'a, T> GetRandom<'a, T> for Vec<T> {
    fn get_random(&'a self) -> Option<&'a T> {
        if self.len() == 0 {
            return None;
        }
        let i = rand::random::<usize>() % self.len();

        Some(&self[i])
    }
}
