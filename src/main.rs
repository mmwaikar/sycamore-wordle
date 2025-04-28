mod models;
mod view_models;
mod word_logic;
mod game_logic;
mod views;

use models::{game::{Game, GameBoard}, game_grid::GameGrid};
use sycamore::prelude::*;
use view_models::{keyboard::Keyboard, position_context::PositionContext};
use views::{game_grid_view::GameGridView, keyboard_view::KeyboardView};

#[component]
fn App() -> View {
    let position_context = PositionContext::new();
    let game_grid = GameGrid::new();
    let keyboard = Keyboard::new();
    let game = Game::new("whale".to_string());
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
