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
use view_models::{hide_context::HideContext, keyboard::Keyboard, position_context::PositionContext};
use views::{game_grid_view::GameGridView, keyboard_view::KeyboardView};
use word_lib::get_random_word;

#[component]
fn App() -> View {
    let position_context = PositionContext::new();
    let hide_context = HideContext::new();
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
    provide_context(hide_context);
    provide_context(game_grid);
    provide_context(keyboard);
    provide_context(game_board);

    let gbc = use_context::<GameBoard>();

    view! {
        div {
            h1 { "Wordle in Rust using Sycamore!" }
            h3(class=gbc.game_status.get().css_class()) { "Game Status: " (gbc.game_status.get().status()) }
            h4(hidden=hide_context.get()) { "Invalid word, please try again!" }
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
