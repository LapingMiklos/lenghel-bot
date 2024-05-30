use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Gifs(Vec<String>);

impl Gifs {
    pub fn get(&self) -> String {
        let i = rand::random::<usize>() % self.0.len();

        self.0[i].clone()
    }
}
