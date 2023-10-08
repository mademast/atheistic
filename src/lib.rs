use std::sync::OnceLock;

use ahash::{AHashMap, AHashSet};
use bible::{Bible, Book, Testament};
use unicode_segmentation::UnicodeSegmentation;

pub mod bible;

#[derive(Copy, Clone)]
pub struct WordMap {
    testament: &'static Testament<'static>,
    book: &'static Book<'static>,
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
    let words = input
        .unicode_words()
        .map(|word| word.to_lowercase())
        .filter(|word| !IGNORE_LIST.contains(&word.as_str()))
        .collect::<Vec<_>>();

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
    pub section: String,
}

pub fn where_in_the_bible(input: &str) -> Option<WhereWasWord> {
    let words = input
        .unicode_words()
        .map(|word| word.to_lowercase())
        .filter(|word| !IGNORE_LIST.contains(&word.as_str()))
        .collect::<Vec<_>>();
    let matching = matching_words(&words);

    let (word, first) = match matching
        .first()
        .and_then(|(word, places)| places.first().map(|wm| (*word, wm)))
    {
        None => return None,
        Some(place) => place,
    };

    let threshold = 80 * 5;
    let bible = first.book.verses.to_lowercase();
    let index = bible.find(word.as_str())?;
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

    Some(WhereWasWord {
        testament: first.testament.title,
        book: first.book.title,
        section: bible[min_index..=max_index].to_string(),
    })
}

pub fn get_bible() -> &'static Bible {
    BIBLE.get_or_init(|| bible::parse().unwrap())

    /*BIBLE.get_or_init(|| {
        get_lowercase_bible()
            .unicode_words()
            .map(|str| str.to_string())
            .collect::<AHashSet<_>>()
    })*/
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
            let set = book
                .verses
                .to_lowercase()
                .unicode_words()
                .map(<_>::to_owned)
                .collect::<AHashSet<_>>();

            for k in set {
                let wordmap = WordMap { testament, book };

                map.entry(k)
                    .and_modify(|vec| vec.push(wordmap))
                    .or_insert(vec![wordmap]);
            }
        }

        map
    };

    BIBLE_MAP.get_or_init(init)
}
