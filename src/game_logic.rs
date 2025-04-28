use crate::{
    models::{
        constants::MAX_ATTEMPTS,
        enums::GameStatus,
        game::Game,
        guess::{Guess, GuessOutcome},
    },
    word_logic::{are_all_correct, get_each_alphabet_status},
};

pub fn get_guess_outcome(game: &Game, guess: &Guess) -> GuessOutcome {
    let alphabets_with_statuses =
        get_each_alphabet_status(game.word_to_guess.as_str(), &guess.word);
    let all_correct = are_all_correct(&alphabets_with_statuses);

    let game_status = match all_correct {
        true => GameStatus::Won,
        _ => {
            if guess.number < MAX_ATTEMPTS as u8 {
                GameStatus::InProgress
            } else {
                GameStatus::Lost
            }
        }
    };

    GuessOutcome::new(guess.clone(), alphabets_with_statuses, game_status)
}
