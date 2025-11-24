const FULL_TEXT: &str = include_str!("../bible.txt");
// A state change denotes the break between books or the index from the first editions
const STATE_CHANGE: &str = "\r\n\r\n\r\n\r\n\r\n";
// The delimiter between editions (Old Testament/New Testament)
const TESTAMENT_DELIMITER: &str = "***\r\n";
// The title delimiter breaks the book title from the verses.
const BOOK_TITLE_DELIMITER: &str = "\r\n\r\n\r\n";

// Parse the bible into the Old and New Testament
pub fn parse() -> Result<Bible, BiblicalError> {
    let (_raw_index, bible_text) = FULL_TEXT
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
    pub verses: Vec<Verse<'b>>,
}

impl<'b> Book<'b> {
    fn parse(raw: &'b str) -> Result<Self, BiblicalError> {
        match raw.split_once(BOOK_TITLE_DELIMITER) {
            None => Err(BiblicalError::BookMissingTitle),
            Some((title, content)) => {
                let title = title.trim();

                let mut verses = vec![];
                let mut last_verse = (0, 0);
                let mut wrk = content.trim();
                loop {
                    let (verse, remaining) = match Verse::parse(wrk) {
                        Ok(ok) => ok,
                        Err(e) => {
                            eprintln!("failed to parse verse: {e}\nbook: {title}\nlast verse: {last_verse:?}");
                            panic!()
                        }
                    };
                    last_verse = (verse.chapter, verse.verse);

                    verses.push(verse);
                    if remaining.is_empty() {
                        break;
                    } else {
                        wrk = remaining;
                    }
                }

                return Ok(Book { title, verses });
            }
        }
    }
}

pub struct Verse<'v> {
    pub chapter: usize,
    pub verse: usize,
    pub content: &'v str,
}

impl<'v> Verse<'v> {
    fn parse(book: &'v str) -> Result<(Self, &'v str), BiblicalError> {
        // the first colon should always be the verse number
        let colon_idx = match book.find(':') {
            None => return Err(BiblicalError::VerseExtraText),
            // Start of line, not a chapter marker
            Some(0) => return Err(BiblicalError::VerseNoChapter),
            Some(idx) => idx,
        };

        // there should not be extra text in the verse before the chapter
        let chapter_raw = &book[..colon_idx];
        let next_white = book.find(|c: char| c.is_whitespace()).unwrap();
        let verse_raw = &book[colon_idx + 1..next_white];

        // loop until we find a colon with a non-white character on each side
        // that is the next chapter marker

        let end_idx = {
            let mut curr_idx = next_white;

            loop {
                let next_colon = match book[curr_idx..].find(':') {
                    None => break book.len(),
                    Some(n) if n + curr_idx == book.len() - 1 => {
                        // At the end of the book
                        break book.len() - 1;
                    }
                    Some(n) => curr_idx + n,
                };

                let mut colon_three = book[next_colon - 1..next_colon + 2].chars();
                let before_numeric = if let Some(ch) = colon_three.next() {
                    ch.is_numeric()
                } else {
                    unreachable!();
                };

                let after_numeric = if let Some(ch) = colon_three.skip(1).next() {
                    ch.is_numeric()
                } else {
                    unreachable!()
                };

                if before_numeric && after_numeric {
                    // We've found the next verse marker, locate where it starts
                    let start_next_marker = book[..next_colon]
                        .char_indices()
                        .rfind(|(_, ch)| ch.is_whitespace())
                        .unwrap()
                        .0;

                    break start_next_marker + 1;
                } else {
                    curr_idx = next_colon + 1;
                }
            }
        };

        let content = &book[next_white..end_idx];
        let remaining = &book[end_idx..];

        let chapter = usize::from_str_radix(chapter_raw, 10).unwrap();
        let verse = usize::from_str_radix(verse_raw, 10).unwrap();

        Ok((
            Self {
                chapter,
                verse,
                content,
            },
            remaining,
        ))
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
    #[error("extra text left not belonging to a verse!")]
    VerseExtraText,
    #[error("verse missing a chapter number!")]
    VerseNoChapter,
}

#[cfg(test)]
mod test {
    use crate::bible::Verse;

    #[test]
    fn lol() {
        let text = "\
1:13 And the Egyptians made the children of Israel to serve with
rigour: 1:14 And they made their lives bitter with hard bondage, in
morter, and in brick, and in all manner of service in the field: all
their service, wherein they made them serve, was with rigour.";

        let (v, r) = Verse::parse(text).unwrap();
        assert_eq!(1, v.chapter);
        assert_eq!(13, v.verse);
        assert_eq!(&text[4..4 + 69], v.content);

        let (v2, r) = Verse::parse(r).unwrap();
        assert_eq!(1, v2.chapter);
        assert_eq!(14, v2.verse);

        let (v2, r) = Verse::parse(r).unwrap();
        assert_eq!(1, v2.chapter);
        assert_eq!(14, v2.verse);
    }
}
