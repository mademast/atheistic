use std::sync::OnceLock;

use ahash::{AHashMap, AHashSet};
use bible::{Bible, Book, Testament};
use lazy_regex::regex;
use unicode_segmentation::UnicodeSegmentation;

use crate::bible::Verse;

pub mod bible;

#[derive(Copy, Clone)]
pub struct WordMap {
    testament: &'static Testament<'static>,
    book: &'static Book<'static>,
    verse: &'static Verse<'static>,
}

static BIBLE: OnceLock<Bible> = OnceLock::new();
static BIBLE_MAP: OnceLock<AHashMap<String, Vec<WordMap>>> = OnceLock::new();
const IGNORE_LIST: &[&str] = &[
    "the", "be", "to", "of", "and", "a", "in", "that", "have", "i", "it", "for", "not", "on",
    "with", "as", "at",
];

pub fn are_any_of_these_words_in_the_bible(input: &str, threshold: usize) -> bool {
    ratio_of_words_in_the_bible(input, threshold) > 0.0
}

pub fn ratio_of_words_in_the_bible(input: &str, threshold: usize) -> f64 {
    let words = process_input(input);

    if words.len() < threshold {
        return 1.0; //technically all of the input words are in the bible
    }

    let bible = get_bible_map();
    let words_in_the_bible = words
        .iter()
        .filter(|&word| bible.contains_key(word))
        .count();

    words_in_the_bible as f64 / words.len() as f64
}

fn process_input(input: &str) -> AHashSet<String> {
    let regex = regex!(r"(https?://)?([\da-z\.-]+)\.([a-z\.]{2,6})([\/\w\.-]*)*\/?");
    regex
        .replace_all(input, " ")
        .to_lowercase()
        .unicode_words()
        .filter(|word| !IGNORE_LIST.contains(word))
        .map(|word| word.into())
        .collect::<AHashSet<String>>()
}

fn matching_words(words: &[String]) -> Vec<(&String, &'static Vec<WordMap>)> {
    let bible = get_bible_map();
    words
        .iter()
        .filter_map(|word| bible.get(word).map(|wm| (word, wm)))
        .collect()
}

pub fn what_words_are_in_the_bible(input: &str) -> Vec<String> {
    let words = input
        .unicode_words()
        .map(|word| word.to_lowercase())
        .filter(|word| !IGNORE_LIST.contains(&word.as_str()))
        .collect::<Vec<_>>();
    let matching = matching_words(&words);

    matching
        .into_iter()
        .map(|(word, _)| word.to_owned())
        .collect()
}

pub struct WhereWasWord {
    pub testament: &'static str,
    pub book: &'static str,
    pub verse: &'static Verse<'static>,
}

pub fn where_in_the_bible(input: &str) -> Option<WhereWasWord> {
    let words = input
        .unicode_words()
        .map(|word| word.to_lowercase())
        .filter(|word| !IGNORE_LIST.contains(&word.as_str()))
        .collect::<Vec<_>>();
    let matching = matching_words(&words);

    // We only show the !where for the first match, I guess
    let (word, first) = match matching
        .first()
        .and_then(|(word, places)| places.first().map(|wm| (*word, wm)))
    {
        None => return None,
        Some(place) => place,
    };

    Some(WhereWasWord {
        testament: first.testament.title,
        book: first.book.title,
        verse: first.verse,
    })
}

pub fn get_bible() -> &'static Bible {
    BIBLE.get_or_init(|| bible::parse().unwrap())
}

pub fn get_bible_map() -> &'static AHashMap<String, Vec<WordMap>> {
    let init = || {
        let bible = get_bible();

        let mut map: AHashMap<String, Vec<WordMap>> = AHashMap::default();

        let books = bible
            .old
            .books
            .iter()
            .map(|book| (&bible.old, book))
            .chain(bible.new.books.iter().map(|book| (&bible.new, book)));

        for (testament, book) in books {
            for verse in &book.verses {
                let set = verse
                    .content
                    .to_lowercase()
                    .unicode_words()
                    .map(<_>::to_owned)
                    .collect::<AHashSet<_>>();

                for k in set {
                    let wordmap = WordMap {
                        testament,
                        book,
                        verse,
                    };

                    map.entry(k)
                        .and_modify(|vec| vec.push(wordmap))
                        .or_insert(vec![wordmap]);
                }
            }
        }

        map
    };

    BIBLE_MAP.get_or_init(init)
}
