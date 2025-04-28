use sycamore::prelude::*;

use crate::models::position::Position;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PositionContext(Signal<Position>);

impl PositionContext {
    pub fn new() -> Self {
        Self(create_signal(Position::init()))
    }

    pub fn get_position(self) -> Position {
        self.0.get()
    }

    pub fn set_next_position(self, next_position: Position) {
        self.0.set(next_position);
    }
}
