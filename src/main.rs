mod game_logic;
mod models;
mod view_models;
mod views;
mod word_lib;
mod word_logic;

use models::{
    game::{Game, GameBoard},
    game_grid::GameGrid,
};
use sycamore::prelude::*;
use view_models::{keyboard::Keyboard, position_context::PositionContext};
use views::{game_grid_view::GameGridView, keyboard_view::KeyboardView};
use word_lib::get_random_word;

#[component]
fn App() -> View {
    let position_context = PositionContext::new();
    let game_grid = GameGrid::new();
    let keyboard = Keyboard::new();

    let dev_mode = true;
    let game = match dev_mode {
        true => Game::new("whale".to_string()),
        false => Game::new(get_random_word()),
    };
    console_log!("game: {:?}", game);
    let game_board = GameBoard::new(game);

    provide_context(position_context);
    provide_context(game_grid);
    provide_context(keyboard);
    provide_context(game_board);

    let gbc = use_context::<GameBoard>();

    view! {
        div {
            h1 { "Welcome to your favorite Wordle game written in Rust using Sycamore!" }
            h3 { "Game Status: " (gbc.game_status.status()) }
        }

        div(class="game") {
            p{}
            GameGridView()

            p{}
            KeyboardView()
        }
    }
}

fn main() {
    sycamore::render(App);
}
