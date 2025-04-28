use crate::models::{alphabet_with_status::AlphabetWithStatus, enums::AlphabetStatus};

pub fn get_alphabet_status(word: &str, actual: char, guess: char) -> AlphabetWithStatus {
    if actual == guess {
        AlphabetWithStatus::with_status(guess, AlphabetStatus::CorrectPosition)
    } else if word.contains(guess) {
        AlphabetWithStatus::with_status(guess, AlphabetStatus::IncorrectPosition)
    } else {
        AlphabetWithStatus::with_status(guess, AlphabetStatus::Absent)
    }
}

pub fn get_each_alphabet_status(actual: &str, guess: &str) -> Vec<AlphabetWithStatus> {
    actual
        .chars()
        .zip(guess.chars())
        .map(|(a, g)| get_alphabet_status(actual, a, g))
        .collect()
}

pub fn are_all_correct(alphabets_with_statuses: &Vec<AlphabetWithStatus>) -> bool {
    alphabets_with_statuses
        .iter()
        .all(|aws| aws.status == AlphabetStatus::CorrectPosition)
}
