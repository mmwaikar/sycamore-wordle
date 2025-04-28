use crate::view_models::grid_alphabet::GridAlphabet;

use super::{constants::{MAX_ATTEMPTS, WORD_LENGTH}, position::Position};

#[derive(Clone, Debug, PartialEq)]
pub struct GameGrid {
    pub char_grid: [[GridAlphabet; WORD_LENGTH]; MAX_ATTEMPTS],
}

impl GameGrid {
    pub fn new() -> Self {
        let char_grid = Self::get_grid();
        Self { char_grid }
    }
    
    fn get_grid() -> [[GridAlphabet; WORD_LENGTH]; MAX_ATTEMPTS] {
        let mut alphabets_with_statuses = [[GridAlphabet::init(); WORD_LENGTH]; MAX_ATTEMPTS];

        for row in 0..MAX_ATTEMPTS {
            for col in 0..WORD_LENGTH {
                let position = Position::new(row as u8, col as u8);
                let aws = GridAlphabet::with_position(' ', position);
                alphabets_with_statuses[row][col] = aws;
            }
        }

        alphabets_with_statuses
    }
}
