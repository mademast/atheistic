use std::{collections::HashSet, sync::OnceLock};

use unicode_segmentation::UnicodeSegmentation;

static BIBLE: OnceLock<HashSet<String>> = OnceLock::new();

pub fn are_any_of_these_words_in_the_bible(input: &str) -> bool {
    ratio_of_words_in_the_bible(input) > 0.0
}

pub fn ratio_of_words_in_the_bible(input: &str) -> f64 {
    let bible = BIBLE.get_or_init(|| {
        include_str!("../bible.txt")
            .to_lowercase()
            .unicode_words()
            .map(|str| str.to_string())
            .collect::<HashSet<_>>()
    });
    let words = input.unicode_words().collect::<Vec<_>>();
    if words.is_empty() {
        return 1.0; //technically all of the input words are in the bible
    }
    let words_in_the_bible = words.iter().filter(|&&word| bible.contains(word)).count();
    words_in_the_bible as f64 / words.len() as f64
}
