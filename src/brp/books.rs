use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fmt::{self, Display},
    str::FromStr,
    u16,
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub enum Book {
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    FirstSamuel,
    SecondSamuel,
    FirstKings,
    SecondKings,
    FirstChronicles,
    SecondChronicles,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    SongOfSolomon,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    FirstCorinthians,
    SecondCorinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    FirstThessalonians,
    SecondThessalonians,
    FirstTimothy,
    SecondTimothy,
    Titus,
    Philemon,
    Hebrews,
    James,
    FirstPeter,
    SecondPeter,
    FirstJohn,
    SecondJohn,
    ThirdJohn,
    Jude,
    Revelation,
}

pub struct BookInfo {
    pub display_to_book: HashMap<&'static str, Book>,
    pub book_to_display: HashMap<Book, &'static str>,
    pub chapter_map: HashMap<Book, u16>,
    pub index_map: HashMap<Book, usize>,
}

macro_rules! book_info {
    ($($book:expr => $name:expr, $chapters:expr, $index:expr),*) => {{
        let mut display_to_book = HashMap::new();
        let mut book_to_display = HashMap::new();
        let mut chapter_map = HashMap::new();
        let mut index_map = HashMap::new();
        $(display_to_book.insert($name, $book);)*
        $(book_to_display.insert($book, $name);)*
        $(chapter_map.insert($book, $chapters);)*
        $(index_map.insert($book, $index);)*
        BookInfo {
            display_to_book,
            book_to_display,
            chapter_map,
            index_map
        }
    }};
}

lazy_static! {
    static ref BOOK_INFO: BookInfo = book_info![
        Book::Genesis => "Genesis", 50, 1,
        Book::Exodus => "Exodus", 40, 2,
        Book::Leviticus => "Leviticus", 27, 3,
        Book::Numbers => "Numbers", 36, 4,
        Book::Deuteronomy => "Deuteronomy", 34, 5,
        Book::Joshua => "Joshua", 24, 6,
        Book::Judges => "Judges", 21, 7,
        Book::Ruth => "Ruth", 4, 8,
        Book::FirstSamuel => "1 Samuel", 31, 9,
        Book::SecondSamuel => "2 Samuel", 24, 10,
        Book::FirstKings => "1 Kings", 22, 11,
        Book::SecondKings => "2 Kings", 25, 12,
        Book::FirstChronicles => "1 Chronicles", 29, 13,
        Book::SecondChronicles => "2 Chronicles", 36, 14,
        Book::Ezra => "Ezra", 10, 15,
        Book::Nehemiah => "Nehemiah", 13, 16,
        Book::Esther => "Esther", 10, 17,
        Book::Job => "Job", 42, 18,
        Book::Psalms => "Psalms", 150, 19,
        Book::Proverbs => "Proverbs", 31, 20,
        Book::Ecclesiastes => "Ecclesiastes", 12, 21,
        Book::SongOfSolomon => "Song of Solomon", 8, 22,
        Book::Isaiah => "Isaiah", 66, 23,
        Book::Jeremiah => "Jeremiah", 52, 24,
        Book::Lamentations => "Lamentations", 5, 25,
        Book::Ezekiel => "Ezekiel", 48, 26,
        Book::Daniel => "Daniel", 12, 27,
        Book::Hosea => "Hosea", 14, 28,
        Book::Joel => "Joel", 3, 29,
        Book::Amos => "Amos", 9, 30,
        Book::Obadiah => "Obadiah", 1, 31,
        Book::Jonah => "Jonah", 4, 32,
        Book::Micah => "Micah", 7, 33,
        Book::Nahum => "Nahum", 3, 34,
        Book::Habakkuk => "Habakkuk", 3, 35,
        Book::Zephaniah => "Zephaniah", 3, 36,
        Book::Haggai => "Haggai", 2, 37,
        Book::Zechariah => "Zechariah", 14, 38,
        Book::Malachi => "Malachi", 4, 39,
        Book::Matthew => "Matthew", 28, 40,
        Book::Mark => "Mark", 16, 41,
        Book::Luke => "Luke", 24, 42,
        Book::John => "John", 21, 43,
        Book::Acts => "Acts", 28, 44,
        Book::Romans => "Romans", 16, 45,
        Book::FirstCorinthians => "1 Corinthians", 16, 46,
        Book::SecondCorinthians => "2 Corinthians", 13, 47,
        Book::Galatians => "Galatians", 6, 48,
        Book::Ephesians => "Ephesians", 6, 49,
        Book::Philippians => "Philippians", 4, 50,
        Book::Colossians => "Colossians", 4, 51,
        Book::FirstThessalonians => "1 Thessalonians", 5, 52,
        Book::SecondThessalonians => "2 Thessalonians", 3, 53,
        Book::FirstTimothy => "1 Timothy", 6, 54,
        Book::SecondTimothy => "2 Timothy", 4, 55,
        Book::Titus => "Titus", 3, 56,
        Book::Philemon => "Philemon", 1, 57,
        Book::Hebrews => "Hebrews", 13, 58,
        Book::James => "James", 5, 59,
        Book::FirstPeter => "1 Peter", 5, 60,
        Book::SecondPeter => "2 Peter", 3, 61,
        Book::FirstJohn => "1 John", 5, 62,
        Book::SecondJohn => "2 John", 1, 63,
        Book::ThirdJohn => "3 John", 1, 64,
        Book::Jude => "Jude", 1, 65,
        Book::Revelation => "Revelation", 22, 66
    ];
}

impl Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", BOOK_INFO.book_to_display[self])
    }
}

impl FromStr for Book {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BOOK_INFO
            .display_to_book
            .get(s)
            .cloned()
            .ok_or("can't parse str to book")?)
    }
}

impl Book {
    pub fn total_chapters(&self) -> u16 {
        BOOK_INFO.chapter_map[self]
    }

    pub fn index(&self) -> usize {
        BOOK_INFO.index_map[self]
    }
}

pub struct ChapterInfo {
    pub book: Book,
    pub chapter: i64,
}

/// (ChapterInfo, Total Chapters)
pub fn get_day_plan(books: &[Book], day: i64) -> (ChapterInfo, i64) {
    let total_chapters = books.iter().fold(0, |sum, val| sum + val.total_chapters()) as i64;

    let remainder = day % total_chapters;
    let mut r = remainder;
    for book in books {
        if r == 0 {
            let last = books.last().expect("no last element");
            return (
                ChapterInfo {
                    book: last.clone(),
                    chapter: last.total_chapters() as i64,
                },
                total_chapters,
            );
        }
        if r <= book.total_chapters() as i64 {
            return (
                ChapterInfo {
                    book: book.clone(),
                    chapter: r as i64,
                },
                total_chapters,
            );
        }
        r -= book.total_chapters() as i64;
    }

    tracing::trace!("unreachable");
    unreachable!()
}
