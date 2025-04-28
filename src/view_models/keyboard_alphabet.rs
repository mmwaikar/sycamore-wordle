use sycamore::prelude::*;

use crate::models::enums::AlphabetStatus;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KeyboardAlphabet {
    pub alphabet: char,
    pub row: u8,
    pub status: Signal<AlphabetStatus>,
}

impl KeyboardAlphabet {
    pub fn init() -> Self {
        Self {
            alphabet: ' ',
            row: 1,
            status: create_signal(AlphabetStatus::default()),
        }
    }

    pub fn new(alphabet: char, row: u8) -> Self {
        Self {
            alphabet,
            row,
            status: create_signal(AlphabetStatus::default()),
        }
    }
}
