const WORD_LENGTH: usize = 5;
const ALL_WORDS: &str = include_str!("words.txt");

fn sanitize_word(word: &str) -> String {
    word.trim()
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()
}

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
