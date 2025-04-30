use rand::Rng;

const WORD_LENGTH: usize = 5;
const ALL_WORDS: &str = include_str!("words.txt");

pub fn get_5_letter_words() -> Vec<String> {
    ALL_WORDS
        .split('\n')
        .skip(2)
        .map(sanitize_word)
        .filter(|line| line.len() == WORD_LENGTH)
        .collect()
}

pub fn is_valid_word(word: &str) -> bool {
    get_5_letter_words().contains(&sanitize_word(word))
}

pub fn get_random_word() -> String {
    let five_letter_words = get_5_letter_words();
    let mut rng = rand::rng();
    let n = rng.random_range(1..=five_letter_words.len());
    let word = &five_letter_words[n];
    word.clone()
}

fn sanitize_word(word: &str) -> String {
    word.trim()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()
}
