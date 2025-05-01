use sycamore::prelude::*;
use sycamore::web::wasm_bindgen::prelude::*;
use web_sys::{HtmlButtonElement, MouseEvent};

use crate::{
    game_logic::get_guess_outcome,
    models::{
        constants::WORD_LENGTH, enums::GameStatus, game::GameBoard, game_grid::GameGrid, guess::{Guess, GuessOutcome}, position::Position,
    },
    view_models::{keyboard::Keyboard, position_context::PositionContext},
};

#[component]
pub fn KeyboardView() -> View {
    // global contexts
    let position_context = use_context::<PositionContext>();
    let game_grid = use_context::<GameGrid>();
    let keyboard = use_context::<Keyboard>();
    let game_board = use_context::<GameBoard>();

    // local signals
    let chars_entered = create_signal(0);
    let num_guesses = create_signal(0);
    // let current_word = create_signal("".to_string());

    create_effect(move || {
        let value = chars_entered.get();
        console_log!("chars_entered = {value}");

        // if value > 0 {
        //     &game_board.update(GameStatus::InProgress);
        // }
    });

    let disable_backspace_key = move || chars_entered.get() == 0;

    let disable_enter_key = move || {
        if chars_entered.get() == 0 {
            true
        } else {
            chars_entered.get() % WORD_LENGTH != 0
        }
    };

    let on_char_click = move |e: MouseEvent| {
        // on click of every alphabet, we are moving the position to the next position
        let position = position_context.get_position();
        let next_position = position.get_next_position();
        position_context.set_next_position(next_position);

        game_grid.char_grid[position.row as usize][position.col as usize]
            .alphabet
            .set(
                e.target()
                    .unwrap()
                    .dyn_into::<HtmlButtonElement>()
                    .unwrap()
                    .inner_text(),
            );
        chars_entered.set(chars_entered.get() + 1);
    };

    let on_backspace = move |_| {
        let position = position_context.get_position();
        let prev_position = position.get_prev_position();
        // console_log!("current: {:?}, prev_position: {:?}", position, prev_position.tuple());
        position_context.set_next_position(prev_position);

        // since position has already been moved to the next position, we need to remove the
        // character at the previouos position
        game_grid.char_grid[prev_position.row as usize][prev_position.col as usize]
            .alphabet
            .set("".to_string());
        chars_entered.set(chars_entered.get() - 1);
    };

    let on_enter = move |_| {
        let position = position_context.get_position();
        let prev_position = position.get_prev_position();
        let guess_count = num_guesses.get();

        let str_array = game_grid.char_grid[prev_position.row as usize];
        let word = str_array
            .iter()
            .map(|aws| aws.alphabet.get_clone_untracked())
            .collect::<Vec<String>>()
            .join("");
        let guess = Guess::new(guess_count + 1, word);
        // console_log!("guess: {:?}", guess);
        let guess_outcome = get_guess_outcome(&game_board.game, &guess);

        // also increment the number of guesses
        num_guesses.set(guess_count + 1);
        &game_board.game_status.set(guess_outcome.intermediate_game_status);

        if guess_outcome.intermediate_game_status == GameStatus::Won {
            update(&guess_outcome, &game_grid, &keyboard, &prev_position);
            // for (i, aws) in guess_outcome.alphabets_with_statuses.iter().enumerate() {
            //     game_grid.char_grid[prev_position.row as usize][i].update_status(aws.status);
            // }

            // for (i, c) in guess_outcome.guess.word.chars().enumerate() {
            //     let aws = guess_outcome.alphabets_with_statuses[i];
            //     let ka = keyboard.keyboard.iter().find(|ka| ka.alphabet == c);
            //     if let Some(ka) = ka {
            //         ka.status.set(aws.status);
            //     }
            // }
            // console_log!("You won!");
        } else if guess_outcome.intermediate_game_status == GameStatus::Lost {
            update(&guess_outcome, &game_grid, &keyboard, &prev_position);
            // for (i, aws) in guess_outcome.alphabets_with_statuses.iter().enumerate() {
            //     game_grid.char_grid[prev_position.row as usize][i].update_status(aws.status);
            // }

            // for (i, c) in guess_outcome.guess.word.chars().enumerate() {
            //     let aws = guess_outcome.alphabets_with_statuses[i];
            //     let ka = keyboard.keyboard.iter().find(|ka| ka.alphabet == c);
            //     if let Some(ka) = ka {
            //         ka.status.set(aws.status);
            //     }
            // }
            // console_log!("You lost!");
        } else {
            update(&guess_outcome, &game_grid, &keyboard, &prev_position);
            // for (i, aws) in guess_outcome.alphabets_with_statuses.iter().enumerate() {
            //     game_grid.char_grid[prev_position.row as usize][i].update_status(aws.status);
            // }

            // for (i, c) in guess_outcome.guess.word.chars().enumerate() {
            //     let aws = guess_outcome.alphabets_with_statuses[i];
            //     let ka = keyboard.keyboard.iter().find(|ka| ka.alphabet == c);
            //     if let Some(ka) = ka {
            //         ka.status.set(aws.status);
            //     }
            // }
        }
    };

    let fr_views = keyboard
        .keyboard
        .into_iter()
        .filter(|ka| ka.row == 1)
        .map(|ka| {
            view! {
                button(class=ka.status.get().css_class(), on:click=on_char_click) {
                    (ka.alphabet.to_string())
                }
            }
        })
        .collect::<Vec<View>>();

    let sr_views = keyboard
        .keyboard
        .into_iter()
        .filter(|ka| ka.row == 2)
        .map(|ka| {
            view! {
                button(class=ka.status.get().css_class(), on:click=on_char_click) {
                    (ka.alphabet.to_string())
                }
            }
        })
        .collect::<Vec<View>>();

    let tr_views = keyboard
        .keyboard
        .into_iter()
        .filter(|ka| ka.row == 3)
        .map(|ka| {
            view! {
                button(class=ka.status.get().css_class(), on:click=on_char_click) {
                    (ka.alphabet.to_string())
                }
            }
        })
        .collect::<Vec<View>>();

    view! {
        div(class="keyboard") {
            div(class="row") {
                (fr_views)
            }

            div(class="row") {
                (sr_views)
                button(class="key", on:click=on_backspace, disabled=disable_backspace_key()) {
                    "<-"
                }
            }

            div(class="row") {
                (tr_views)
                button(class="key", on:click=on_enter, disabled=disable_enter_key) {
                    "Enter"
                }
            }
        }
    }
}

fn update(guess_outcome: &GuessOutcome, game_grid: &GameGrid, keyboard: &Keyboard, prev_position: &Position) {
    // update the game grid with the guess outcome
    for (i, aws) in guess_outcome.alphabets_with_statuses.iter().enumerate() {
        game_grid.char_grid[prev_position.row as usize][i].update_status(aws.status);
    }

    // update the keyboard with the guess outcome
    for (i, c) in guess_outcome.guess.word.chars().enumerate() {
        let aws = guess_outcome.alphabets_with_statuses[i];
        let ka = keyboard.keyboard.iter().find(|ka| ka.alphabet == c);
        if let Some(ka) = ka {
            ka.status.set(aws.status);
        }
    }
}
