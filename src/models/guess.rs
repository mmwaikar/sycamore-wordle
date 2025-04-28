use super::{alphabet_with_status::AlphabetWithStatus, enums::GameStatus};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Guess {
    pub number: u8,
    pub word: String,
}

impl Guess {
    pub fn new(number: u8, word: String) -> Guess {
        Guess { number, word }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GuessOutcome {
    pub guess: Guess,
    pub alphabets_with_statuses: Vec<AlphabetWithStatus>,
    pub intermediate_game_status: GameStatus,
}

impl GuessOutcome {
    pub fn new(
        guess: Guess,
        alphabets_with_statuses: Vec<AlphabetWithStatus>,
        intermediate_game_status: GameStatus,
    ) -> GuessOutcome {
        GuessOutcome {
            guess,
            alphabets_with_statuses,
            intermediate_game_status,
        }
    }
}
