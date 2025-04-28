use crate::models::constants::WORD_LENGTH;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub row: u8,
    pub col: u8,
}

impl Position {
    pub fn init() -> Self {
        Self { row: 0, col: 0 }
    }

    pub fn new(row: u8, col: u8) -> Self {
        Self { row, col }
    }

    pub fn get_next_position(&self) -> Self {
        if self.col < (WORD_LENGTH as u8) - 1 {
            Self {
                row: self.row,
                col: self.col + 1,
            }
        } else {
            Self {
                row: self.row + 1,
                col: 0,
            }
        }
    }

    pub fn get_prev_position(&self) -> Self {
        if self.col > 0 {
            Self {
                row: self.row,
                col: self.col - 1,
            }
        } else {
            Self {
                row: self.row - 1,
                col: (WORD_LENGTH as u8) - 1,
            }
        }
    }
}
