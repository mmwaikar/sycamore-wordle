use sycamore::prelude::*;

use crate::models::{enums::GameStatus, game::Game};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GameBoardVM {
    pub game: Game,
    pub game_status: Signal<GameStatus>,
}

impl GameBoardVM {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            game_status: create_signal(GameStatus::NotStarted),
        }
    }
}
