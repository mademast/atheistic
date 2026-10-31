use std::{io::Read, sync::OnceLock};

use lzma_rust2::Lzma2Reader;

const COMPRESSED_WORD_OF_GOD: &[u8] = include_bytes!("../bible.lzma");
static DECOMPRESSED_WORD_OF_GOD: OnceLock<String> = OnceLock::new();

// A state change denotes the break between books or the index from the first editions
const STATE_CHANGE: &str = "\r\n\r\n\r\n\r\n\r\n";
// The delimiter between editions (Old Testament/New Testament)
const TESTAMENT_DELIMITER: &str = "***\r\n";
// The title delimiter breaks the book title from the verses.
const BOOK_TITLE_DELIMITER: &str = "\r\n\r\n\r\n";

fn decompress_gods_word() -> &'static str {
    DECOMPRESSED_WORD_OF_GOD.get_or_init(|| {
        // wc -c bible.txt
        let mut decompressed = Vec::with_capacity(4432182);

        Lzma2Reader::new(COMPRESSED_WORD_OF_GOD, Default::default(), None)
            .read_to_end(&mut decompressed)
            .expect("bible failed to decompress");

        String::from_utf8(decompressed).expect("bible did not decompress into a utf-8 string")
    })
}

// Parse the bible into the Old and New Testament
pub fn parse() -> Result<Bible, BiblicalError> {
    let (_raw_index, bible_text) = decompress_gods_word()
        .split_once(STATE_CHANGE)
        .ok_or(BiblicalError::MissingIndex)?;

    let (old, new) = bible_text
        .split_once(TESTAMENT_DELIMITER)
        .ok_or(BiblicalError::MissingTestamentDelimiter)?;

    Ok(Bible {
        old: Testament::parse(old)?,
        new: Testament::parse(new)?,
    })
}

pub struct Bible {
    pub old: Testament<'static>,
    pub new: Testament<'static>,
}

pub struct Testament<'t> {
    pub title: &'t str,
    pub books: Vec<Book<'t>>,
}

impl<'t> Testament<'t> {
    fn parse(raw: &'t str) -> Result<Self, BiblicalError> {
        let mut splits = raw.split(STATE_CHANGE);
        // well the first one is the title.
        let title = splits
            .next()
            .ok_or(BiblicalError::TestamentMissingTitle)?
            .trim();

        let mut books = vec![];
        for book in splits {
            books.push(Book::parse(book)?);
        }

        Ok(Testament { title, books })
    }
}

pub struct Book<'b> {
    pub title: &'b str,
    //TODO: parse verses into the [Verse] struct, but it's a little difficult
    pub verses: &'b str,
}

impl<'b> Book<'b> {
    fn parse(raw: &'b str) -> Result<Self, BiblicalError> {
        match raw.split_once(BOOK_TITLE_DELIMITER) {
            None => Err(BiblicalError::BookMissingTitle),
            Some((title, content)) => Ok(Book {
                title: title.trim(),
                verses: content,
            }),
        }
    }
}

pub struct Verse {
    pub chapter: usize,
    pub verse: usize,
    pub content: String,
}

fn verse_number(raw: &str) -> Option<(usize, usize)> {
    match raw.find(' ') {
        None => None,
        Some(idx) => match &raw[..idx].split_once(':') {
            None => None,
            Some((chapter, verse)) => {
                let chapter: usize = chapter.trim().parse().unwrap();
                let verse: usize = verse.trim().parse().unwrap();

                Some((chapter, verse))
            }
        },
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BiblicalError {
    #[error("the bible is in an unexpected format! it appears to be missing the index!")]
    MissingIndex,
    #[error("the bible is in an unexpected format! cannot locate the testament delimiter!")]
    MissingTestamentDelimiter,
    #[error("testament missing title!")]
    TestamentMissingTitle,
    #[error("book missing title!")]
    BookMissingTitle,
}
