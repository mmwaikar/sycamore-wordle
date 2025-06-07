mod game_logic;
mod models;
mod view_models;
mod views;
mod word_lib;
mod word_logic;

use models::game::Game;
use sycamore::prelude::*;
use view_models::{
    game_board_vm::GameBoardVM, guess_grid_vm::GuessGridVM, hide_context::HideContext,
    keyboard_vm::KeyboardVM, position_context::PositionContext,
};
use views::{game_grid_view::GameGridView, keyboard_view::KeyboardView};
use word_lib::get_random_word;

#[component]
fn App() -> View {
    let position_context = PositionContext::new();
    let hide_context = HideContext::new();
    let guess_grid_vm = GuessGridVM::new();
    let keyboard_vm = KeyboardVM::new();

    let dev_mode = true;
    let game = match dev_mode {
        true => Game::new("whale".to_string()),
        false => Game::new(get_random_word()),
    };
    console_log!("game: {:?}", game);
    let game_board = GameBoardVM::new(game);

    provide_context(position_context);
    provide_context(hide_context);
    provide_context(guess_grid_vm);
    provide_context(keyboard_vm);
    provide_context(game_board);

    let gbc = use_context::<GameBoardVM>();

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
