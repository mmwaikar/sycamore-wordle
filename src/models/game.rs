#[derive(Clone, Debug, Default, PartialEq)]
pub struct Game {
    pub word_to_guess: String,
}

impl Game {
    pub fn new(word_to_guess: String) -> Game {
        Game { word_to_guess }
    }
}
