use std::{collections::HashSet, sync::OnceLock};

use unicode_segmentation::UnicodeSegmentation;

static BIBLE: OnceLock<HashSet<String>> = OnceLock::new();
static LOWERCASE_BIBLE: OnceLock<String> = OnceLock::new();
const IGNORE_LIST: &[&str] = &[
    "the", "be", "to", "of", "and", "a", "in", "that", "have", "i", "it", "for", "not", "on",
    "with", "as", "at",
];

pub fn are_any_of_these_words_in_the_bible(input: &str, threshold: usize) -> bool {
    ratio_of_words_in_the_bible(input, threshold) > 0.0
}

pub fn ratio_of_words_in_the_bible(input: &str, threshold: usize) -> f64 {
    let words = input
        .unicode_words()
        .map(|word| word.to_lowercase())
        .filter(|word| !IGNORE_LIST.contains(&word.as_str()))
        .collect::<Vec<_>>();
    if words.len() < threshold {
        return 1.0; //technically all of the input words are in the bible
    }
    let bible = get_bible();
    let words_in_the_bible = words.iter().filter(|&word| bible.contains(word)).count();
    words_in_the_bible as f64 / words.len() as f64
}

pub fn where_in_the_bible(pattern: &str) -> Option<&'static str> {
    let threshold = 80 * 5;
    let bible = get_lowercase_bible();
    let index = bible.find(pattern)?;
    let mut min_index = index;
    let mut max_index = index;

    while min_index > 0 && min_index > index - threshold {
        if bible[min_index..=index].contains("\r\n\r\n") {
            break;
        } else {
            min_index -= 1;
        }
    }
    while max_index < bible.len() && max_index < index + threshold {
        if bible[index..=max_index].contains("\r\n\r\n") {
            break;
        } else {
            max_index += 1;
        }
    }

    Some(&bible[min_index..=max_index])
}

pub fn get_bible() -> &'static HashSet<String> {
    BIBLE.get_or_init(|| {
        get_lowercase_bible()
            .unicode_words()
            .map(|str| str.to_string())
            .collect::<HashSet<_>>()
    })
}

pub fn get_lowercase_bible() -> &'static String {
    LOWERCASE_BIBLE.get_or_init(|| include_str!("../bible.txt").to_lowercase())
}
