use super::enums::AlphabetStatus;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AlphabetWithStatus {
    pub alphabet: char,
    pub status: AlphabetStatus,
}

impl AlphabetWithStatus {
    // pub fn init() -> Self {
    //     Self {
    //         alphabet: ' ',
    //         status: AlphabetStatus::default(),
    //     }
    // }

    // pub fn new(alphabet: char) -> Self {
    //     Self {
    //         alphabet,
    //         status: AlphabetStatus::default(),
    //     }
    // }

    pub fn with_status(alphabet: char, status: AlphabetStatus) -> Self {
        Self { alphabet, status }
    }

    // pub fn update_status(&self, status: AlphabetStatus) -> Self {
    //     Self { status, ..*self }
    // }

    // pub fn get_alphabet(&self) -> char {
    //     self.alphabet
    // }
}
