use sycamore::prelude::*;

use crate::models::{enums::AlphabetStatus, position::Position};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GuessAlphabetVM {
    pub alphabet: Signal<String>,
    pub position: Position,
    pub status: Signal<AlphabetStatus>,
}

impl GuessAlphabetVM {
    pub fn init() -> Self {
        Self {
            alphabet: create_signal("".to_string()),
            position: Position::init(),
            status: create_signal(AlphabetStatus::default()),
        }
    }

    pub fn with_position(c: char, position: Position) -> Self {
        Self {
            alphabet: create_signal(c.to_string()),
            position,
            status: create_signal(AlphabetStatus::default()),
        }
    }

    pub fn update_status(&self, status: AlphabetStatus) {
        self.status.set(status);
    }
}
