use crate::{
    models::{
        constants::{MAX_ATTEMPTS, WORD_LENGTH},
        position::Position,
    },
    view_models::guess_alphabet_vm::GuessAlphabetVM,
};

#[derive(Clone, Debug, PartialEq)]
pub struct GuessGridVM {
    pub guess_alphabets: [[GuessAlphabetVM; WORD_LENGTH]; MAX_ATTEMPTS],
}

impl GuessGridVM {
    pub fn new() -> Self {
        let char_grid = Self::init();
        Self { guess_alphabets: char_grid }
    }

    fn init() -> [[GuessAlphabetVM; WORD_LENGTH]; MAX_ATTEMPTS] {
        let mut alphabets_with_statuses = [[GuessAlphabetVM::init(); WORD_LENGTH]; MAX_ATTEMPTS];

        for row in 0..MAX_ATTEMPTS {
            for col in 0..WORD_LENGTH {
                let position = Position::new(row as u8, col as u8);
                let aws = GuessAlphabetVM::with_position(' ', position);
                alphabets_with_statuses[row][col] = aws;
            }
        }

        alphabets_with_statuses
    }
}
