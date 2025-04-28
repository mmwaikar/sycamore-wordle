use super::enums::GameStatus;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Game {
    pub word_to_guess: String,
}

impl Game {
    pub fn new(word_to_guess: String) -> Game {
        Game { word_to_guess }
    }
}

#[derive(Clone, Default)]
pub struct GameBoard {
    pub game: Game,
    pub game_status: GameStatus,
}

impl GameBoard {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            game_status: GameStatus::NotStarted,
        }
    }

    pub fn update(&self, game_status: GameStatus) -> Self {
        Self {
            game_status,
            ..self.clone()
        }
    }

    // pub fn init(
    //     game: Game,
    //     final_game_status: GameStatus,
    // ) -> GameBoard {
    //     GameBoard {
    //         game,
    //         final_game_status,
    //     }
    // }

    // pub fn get_guess_with_outcome(&self, n: u8) -> &str {
    //     self.guesses_with_outcomes[n as usize].guess.word.as_str()
    // }

    // pub fn get_current_guess(&self) -> u8 {
    //     self.guesses_with_outcomes
    //         .iter()
    //         .position(|guess| guess.guess.word.is_empty())
    //         .unwrap_or(MAX_ATTEMPTS as usize) as u8
    // }

    // pub fn push(&mut self, current_guess: u8, c: char) {
    //     let guess = &mut self.guesses_with_outcomes[current_guess as usize].guess;
    //     guess.word.push(c);
    // }
}
