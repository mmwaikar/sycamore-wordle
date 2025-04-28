#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AlphabetStatus {
    #[default]
    None,
    Absent,
    IncorrectPosition,
    CorrectPosition,
}

impl AlphabetStatus {
    pub fn css_class(&self) -> String {
        match self {
            AlphabetStatus::None => "default-char".to_string(),
            AlphabetStatus::Absent => "absent".to_string(),
            AlphabetStatus::IncorrectPosition => "incorrect-position".to_string(),
            AlphabetStatus::CorrectPosition => "correct-position".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum GameStatus {
    #[default]
    NotStarted,
    InProgress,
    Won,
    Lost,
}

impl GameStatus {
    pub fn status(&self) -> String {
        match self {
            GameStatus::NotStarted => "Not started".to_string(),
            GameStatus::InProgress => "In-progress".to_string(),
            GameStatus::Won => "Won".to_string(),
            GameStatus::Lost => "Lost".to_string(),
        }
    }
}
