/*
              Bible Lib
                .---.
           '-.  |   |  .-'
             ___|   |___
        -=  [           ]  =-
            `---.   .---'
         __||__ |   | __||__
         '-..-' |   | '-..-'
           ||   |   |   ||
           ||_.-|   |-,_||
         .-"`   `"`'`   `"-.
       .'                   '. Art by Joan Stark
*/

use std::{collections::HashMap, fmt::Display};

use crate::error::BibleLibError;

pub mod error;

#[cfg(feature = "akjv")]
const AKJV: &str = include_str!("..\\bible_translations\\akjv.txt");
#[cfg(feature = "asv")]
const ASV: &str = include_str!("..\\bible_translations\\asv.txt");
#[cfg(feature = "erv")]
const ERV: &str = include_str!("..\\bible_translations\\erv.txt");
#[cfg(feature = "kjv")]
const KJV: &str = include_str!("..\\bible_translations\\kjv.txt");

/// Different Bible Translations
/// provided by https://openbible.com/
/// https://openbible.com/texts.htm
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Translation {
    /// American King James Version
    #[cfg(feature = "akjv")]
    AmericanKingJames,
    /// American Standard Version
    #[cfg(feature = "asv")]
    AmericanStandard,
    /// English Revised Version
    #[cfg(feature = "erv")]
    EnglishedRevised,
    /// King James Version
    #[cfg(feature = "kjv")]
    KingJames,
    /// For custom translations,
    /// each line must be a verse formatted as: `Book Chapter:Verse Content`
    /// See bible_translations/ for examples
    /// 
    /// `name` is strictly for display purposes
    ///
    /// note: other translations are included in the binary at compile time,
    /// but custom translations are read from the filesystem at runtime
    Custom { name: String, path: String }
}

impl Translation {
    #[doc(hidden)]
    fn get_text(&self) -> Result<String, BibleLibError> {
        match self {

            Self::AmericanKingJames => {
                Ok(AKJV.to_string())
            }
            Self::AmericanStandard => {
                Ok(ASV.to_string())
            }
            Self::EnglishedRevised => {
                Ok(ERV.to_string())
            }
            Self::KingJames => {
                Ok(KJV.to_string())
            }
            Self::Custom { path, .. } => {
                // ensure the file exists
                if !std::path::Path::new(path).exists() {
                    return Err(BibleLibError::InvalidCustomTranslationFile);
                }

                // read the file and return the content
                let result = std::fs::read_to_string(path);
                match result {
                    Ok(content) => Ok(content),
                    Err(e) => Err(BibleLibError::IOError(e))
                }
            }
        }
    }
}

#[cfg(any(feature = "akjv", feature = "asv", feature = "erv", feature = "kjv"))]
impl Default for Translation {
    #[cfg(feature = "akjv")]
    fn default() -> Self {
        Self::AmericanKingJames
    }
    #[cfg(all(not(feature = "akjv"), feature = "asv"))]
    fn default() -> Self {
        Self::AmericanStandard
    }
    #[cfg(all(not(feature = "akjv"), not(feature = "asv"), feature = "erv"))]
    fn default() -> Self {
        Self::EnglishedRevised
    }
    #[cfg(all(not(feature = "akjv"), not(feature = "asv"), not(feature = "erv"), feature = "kjv"))]
    fn default() -> Self {
        Self::KingJames
    }
}

impl Display for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "akjv")]
            Self::AmericanKingJames => write!(f, "American King James Version"),
            #[cfg(feature = "asv")]
            Self::AmericanStandard => write!(f, "American Standard Version"),
            #[cfg(feature = "erv")]
            Self::EnglishedRevised => write!(f, "English Revised Version"),
            #[cfg(feature = "kjv")]
            Self::KingJames => write!(f, "King James Version"),
            Self::Custom { name, .. } => write!(f, "Custom Translation: {}", name),
        }
    }
}

/// Struct representing a Bible verse lookup
/// `book` is not case-sensitive
/// `thru_verse` is optional and used for verse ranges like `John 3:16-18`
/// # Example
/// ```
/// use bible_lib::{Bible, BibleLookup, Translation};
///
/// // get the bible translation
/// let bible = Bible::new(Translation::KingJames).unwrap();
/// // create a lookup for John 3:16
/// let lookup = BibleLookup::new("John", 3, 16);
/// // get the verse text
/// let verse = bible.get_verse(lookup, false).unwrap();
///
/// // print the verse text
/// println!("John 3:16: {}", verse);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BibleLookup {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub thru_verse: Option<u32>,
}

impl BibleLookup {
    /// Create a new BibleLookup instance (single verse)
    /// `book` is not case-sensitive
    /// # Example
    /// ```
    /// use bible_lib::BibleLookup;
    ///
    /// // create a lookup for John 3:16
    /// let lookup = BibleLookup::new("John", 3, 16);
    /// ```
    pub fn new<S: Into<String>>(book: S, chapter: u32, verse: u32) -> Self {
        let book = book.into();
        let book = book.to_lowercase();
        Self {
            book,
            chapter,
            verse,
            thru_verse: None,
        }
    }

    /// Create a new BibleLookup instance (verse range)
    /// # Example
    /// ```
    /// use bible_lib::BibleLookup;
    ///
    /// // create a lookup for Luke 23:39-43
    /// let lookup = BibleLookup::new_range("Luke", 23, 39, 43);
    /// ```
    pub fn new_range<S: Into<String>>(book: S, chapter: u32, verse: u32, thru_verse: u32) -> Self {
        let book = book.into();
        let book = book.to_lowercase();
        Self {
            book,
            chapter,
            verse,
            thru_verse: Some(thru_verse),
        }
    }

    /// Detect Bible verses in a string
    /// Requires the `detection` feature to be enabled
    /// Can return multiple verses if more than one is found
    /// # Example
    /// ```
    /// use bible_lib::{Bible, Translation, BibleLookup};
    ///
    /// // get the bible translation
    /// let bible = Bible::new(Translation::default()).unwrap();
    ///
    /// // create the string to look for verses in
    /// let text = "Show me John 3:16";
    /// // detect verses in the string
    /// let verses = BibleLookup::detect_from_string(text);
    ///
    /// // iterate through the found verses and print them
    /// for verse in verses {
    ///     // get the verse text
    ///     let verse_text = bible.get_verse(verse.clone()).unwrap();
    ///     // print the verse text
    ///     println!("Found verse: {} - {}", verse, verse_text);
    /// }
    /// ```
    #[cfg(feature = "detection")]
    pub fn detect_from_string<S: Into<String>>(lookup: S) -> Vec<Self> {
        let mut verses = Vec::new();

        let lookup = lookup.into();
        let text = lookup.to_lowercase();

        //let regex = regex::Regex::new(r"\b(?:genesis|exodus|leviticus|numbers|deuteronomy|joshua|judges|ruth|1\s?samuel|2\s?samuel|1\s?kings|2\s?kings|1\s?chronicles|2\s?chronicles|ezra|nehemiah|esther|job|psalms|proverbs|ecclesiastes|song\sof\ssolomon|isaiah|jeremiah|lamentations|ezekiel|daniel|hosea|joel|amos|obadiah|jonah|micah|nahum|habakkuk|zephaniah|haggai|zechariah|malachi|matthew|mark|luke|john|acts|romans|1\s?corinthians|2\s?corinthians|galatians|ephesians|philippians|colossians|1\s?thessalonians|2\s?thessalonians|1\s?timothy|2\s?timothy|titus|philemon|hebrews|james|1\s?peter|2\s?peter|1\s?john|2\s?john|3\s?john|jude|revelation)\s+\d+:\d+\b").unwrap();
        let regex = regex::Regex::new(r"\b(?:genesis|exodus|leviticus|numbers|deuteronomy|joshua|judges|ruth|1\s?samuel|2\s?samuel|1\s?kings|2\s?kings|1\s?chronicles|2\s?chronicles|ezra|nehemiah|esther|job|psalms|proverbs|ecclesiastes|song\sof\ssolomon|isaiah|jeremiah|lamentations|ezekiel|daniel|hosea|joel|amos|obadiah|jonah|micah|nahum|habakkuk|zephaniah|haggai|zechariah|malachi|matthew|mark|luke|john|acts|romans|1\s?corinthians|2\s?corinthians|galatians|ephesians|philippians|colossians|1\s?thessalonians|2\s?thessalonians|1\s?timothy|2\s?timothy|titus|philemon|hebrews|james|1\s?peter|2\s?peter|1\s?john|2\s?john|3\s?john|jude|revelation)\s+\d+:\d+(?:-\d+)?\b").unwrap();
        
        for instance in regex.find_iter(&text) {
            let instance = instance.as_str();
            // to handle cases like `1 samuel` and `Song of Solomon`, split by ':' first and then split by whitespace
            let mut parts = instance.split(':');
            // split the first part by whitespace
            let book_chapter = parts.next().unwrap().split_whitespace();
            let count = book_chapter.clone().count();
            let chapter = book_chapter.clone().last().unwrap().parse::<u32>().unwrap();
            let book = book_chapter.take(count - 1).collect::<Vec<&str>>().join(" ").to_lowercase();

            // handle cases where the verse is a range (i.e. `1-3`)
            let verse_part = parts.next().unwrap();
            if verse_part.contains('-') {
                let verse_split = verse_part.split('-');
                let verse = verse_split.clone().next().unwrap().parse::<u32>().unwrap();
                let thru_verse = verse_split.clone().last().unwrap().parse::<u32>().unwrap();
                verses.push(BibleLookup {
                    book,
                    chapter,
                    verse,
                    thru_verse: Some(thru_verse),
                });
            } else {
                let verse = verse_part.parse::<u32>().unwrap();
                verses.push(BibleLookup {
                    book,
                    chapter,
                    verse,
                    thru_verse: None,
                });
            }
        }

        verses
    }

    /// Capitalize the first letter of each word in the book name
    /// Handles cases like `1 samuel` and `song of solomon`
    /// This is used because book names are stored in lowercase for easier lookup
    /// # Example
    /// ```
    /// use bible_lib::BibleLookup;
    /// 
    /// // capitalize book names
    /// let book1 = BibleLookup::capitalize_book(&"john".to_string());
    /// let book2 = BibleLookup::capitalize_book(&"1 samuel".to_string());
    /// 
    /// // print the capitalized book names
    /// println!("Capitalized Book 1: {}", book1); // John
    /// println!("Capitalized Book 2: {}", book2); // 1 Samuel
    /// 
    /// ```
    pub fn capitalize_book(name: &String) -> String {
        // capitalize the first letter of each word in the book name
        // Split the input string by whitespace into words
        name.split_whitespace()
            // For each word, apply the following transformation
            .map(|word| {
                // Convert the word into characters
                let mut chars = word.chars();
                // If there's a first character, convert it to uppercase and concatenate it with the rest of the characters
                if let Some(first_char) = chars.next() {
                    if first_char.is_numeric() {
                        // If the first character is numeric, leave it unchanged
                        first_char.to_string() + &chars.collect::<String>()
                    } else {
                        // If the first character is not numeric, capitalize it
                        first_char.to_uppercase().chain(chars).collect::<String>()
                    }
                } else {
                    // If the word is empty, return an empty string
                    String::new()
                }
            })
            // Collect the transformed words back into a single string, separated by whitespace
            .collect::<Vec<String>>().join(" ")
    }
}

impl Display for BibleLookup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(thru_verse) = self.thru_verse {
            write!(f, "{} {}:{}-{}", Self::capitalize_book(&self.book), self.chapter, self.verse, thru_verse)
        } else {
            write!(f, "{} {}:{}", Self::capitalize_book(&self.book), self.chapter, self.verse)
        }
    }
}

/// Main Bible struct
/// Stores the verses of the Bible for interfacing
/// # Example
/// ```
/// use bible_lib::{Bible, Translation, BibleLookup};
///
/// // get the bible translation
/// let bible = Bible::new(Translation::AmericanStandard).unwrap();
///
/// // create a lookup for John 3:16
/// let lookup = BibleLookup::new("John", 3, 16);
/// // get the verse text
/// let verse = bible.get_verse(lookup, false).unwrap();
///
/// // print the verse text
/// println!("John 3:16: {}", verse);
/// ```
#[derive(Debug, Clone)]
pub struct Bible {
    translation: Translation,
    pub verses: HashMap<String /* Book */,
                HashMap<u32 /* Chapter */,
                HashMap<u32 /* Verse */, String /* Text */>>>,
}

impl Bible {

    #[doc(hidden)]
    fn parse_text(lines: &String) -> HashMap<String, HashMap<u32, HashMap<u32, String>>> {
        let mut verses = HashMap::new();

        for line in lines.lines() {
            // to handle cases like `1 samuel` and `Song of Solomon`, split by ':' first and then split by whitespace
            let mut parts = line.split(':');
            // split the first part by whitespace
            let book_chapter = parts.next().unwrap().split_whitespace();
            let count = book_chapter.clone().count();
            let chapter = book_chapter.clone().last().unwrap().parse::<u32>().unwrap();
            let book = book_chapter.take(count - 1).collect::<Vec<&str>>().join(" ").to_lowercase();

            let verse_text = parts.next().unwrap().split_whitespace();
            let verse = verse_text.clone().next().unwrap().parse::<u32>().unwrap();
            let text = verse_text.clone().skip(1).collect::<Vec<&str>>().join(" ");

            if !verses.contains_key(&book) {
                verses.insert(book.to_string(), HashMap::new());
            }
            if !verses.get_mut(&book).unwrap().contains_key(&chapter) {
                verses.get_mut(&book).unwrap().insert(chapter, HashMap::new());
            }
            verses.get_mut(&book).unwrap().get_mut(&chapter).unwrap().insert(verse, text.to_string());
        }

        verses
    }

    /// Create a new Bible instance with the specified translation
    pub fn new(translation: Translation) -> Result<Self, BibleLibError> {
        let text = translation.get_text()?;
        let verses = Self::parse_text(&text);
        Ok(Self {
            translation,
            verses,
        })
    }

    /// Get the current translation of the Bible instance
    pub fn get_translation(&self) -> &Translation {
        &self.translation
    }

    #[doc(hidden)]
    fn replace_superscript(s: String) -> String {
        s.chars().map(|c| {
            match c {
                '0' => '⁰',
                '1' => '¹',
                '2' => '²',
                '3' => '³',
                '4' => '⁴',
                '5' => '⁵',
                '6' => '⁶',
                '7' => '⁷',
                '8' => '⁸',
                '9' => '⁹',
                _ => c,
            }
        }).collect()
    }

    /// Get the text of a verse or range of verses
    /// `use_superscripts` adds superscript verse numbers for better readability
    /// Returns an error if the verse or chapter is not found
    /// # Example
    /// ```
    /// use bible_lib::{Bible, BibleLookup, Translation};
    ///
    /// // get the bible translation
    /// let bible = Bible::new(Translation::AmericanStandard).unwrap();
    /// // create a lookup for John 3:16
    /// let lookup = BibleLookup::new("John", 3, 16);
    /// // get the verse text
    /// let verse = bible.get_verse(lookup, false).unwrap();
    ///
    /// // print the verse text
    /// println!("John 3:16: {}", verse);
    /// ```
    pub fn get_verse(&self, lookup: BibleLookup, use_superscripts: bool) -> Result<String, BibleLibError> {
        // multiple verse lookup
        if let Some(thru_verse) = lookup.thru_verse {
            let mut verse_text = String::new();

            // iterate through the verses
            for verse in lookup.verse..=thru_verse {
                let Some(chapters) = self.verses.get(&lookup.book) else {
                    return Err(BibleLibError::BookNotFound);
                };
                let Some(verses) = chapters.get(&lookup.chapter) else {
                    return Err(BibleLibError::ChapterNotFound);
                };
                let Some(text) = verses.get(&verse) else {
                    return Err(BibleLibError::VerseNotFound);
                };

                if use_superscripts {
                    verse_text.push_str(&format!("{}{} ", Self::replace_superscript(verse.to_string()), text));
                } else {
                    verse_text.push_str(text);
                }
            }
            return Ok(verse_text.trim().to_string());
        }
        
        // single verse lookup
        let Some(chapters) = self.verses.get(&lookup.book) else {
            return Err(BibleLibError::BookNotFound);
        };
        let Some(verses) = chapters.get(&lookup.chapter) else {
            return Err(BibleLibError::ChapterNotFound);
        };
        let Some(text) = verses.get(&lookup.verse) else {
            return Err(BibleLibError::VerseNotFound);
        };

        if use_superscripts {
            Ok(format!("{}{}", Self::replace_superscript(lookup.verse.to_string()), text))
        } else {
            Ok(text.to_string())
        }
    }

    /// Get the text of an entire chapter as a string
    /// `use_superscripts` adds superscript verse numbers for better readability
    /// Returns an error if the chapter is not found
    /// # Example
    /// ```
    /// use bible_lib::{Bible, BibleLookup, Translation};
    ///
    /// // get the bible translation
    /// let bible = Bible::new(Translation::EnglishedRevised).unwrap();
    /// // get the text of Isaiah chapter 53
    /// let chapter_text = bible.get_chapter("Isaiah", 53, true).unwrap();
    ///
    /// // print the chapter text
    /// println!("Isaiah 53: {}", chapter_text);
    /// ```
    pub fn get_chapter(&self, book: &str, chapter: u32, use_superscripts: bool) -> Result<String, BibleLibError> {
        let mut chapter_text = String::new();
        // sort the verses by verse number
        let Some(chapters) = self.verses.get(book) else {
            return Err(BibleLibError::BookNotFound);
        };
        let Some(verses) = chapters.get(&chapter) else {
            return Err(BibleLibError::ChapterNotFound);
        };
        let mut verses = verses.iter().collect::<Vec<(&u32, &String)>>();
        verses.sort_by(|a, b| a.0.cmp(b.0));
        for (verse, text) in verses {
            let verse_designation = Self::replace_superscript(verse.to_string());
            if use_superscripts {
                chapter_text.push_str(&format!("{}{} ", verse_designation, text));
            } else {
                chapter_text.push_str(&format!("{} ", text));
            }
        }
        Ok(chapter_text)
    }

    /// Get a list of all books in the Bible
    /// # Example
    /// ```
    /// use bible_lib::{Bible, Translation};
    ///
    /// // get the bible translation
    /// let bible = Bible::new(Translation::default()).unwrap();
    ///
    /// // get the list of books
    /// let books = bible.get_books();
    /// // print the list of books
    /// println!("Books in the Bible: {:?}", books);
    /// ```
    pub fn get_books(&self) -> Vec<String> {
        self.verses.keys().map(|s| s.to_string()).collect()
    }

    /// Get a list of all chapters in a book
    /// # Example
    /// ```
    /// use bible_lib::{Bible, Translation};
    ///
    /// // get the bible translation
    /// let bible = Bible::new(Translation::default()).unwrap();
    ///
    /// // get the list of chapters in Revelation
    /// let chapters = bible.get_chapters("Revelation").unwrap();
    /// // print the list of chapters
    /// println!("Chapters in Revelation: {:?}", chapters);
    /// ```
    pub fn get_chapters(&self, book: &str) -> Result<Vec<u32>, BibleLibError> {
        if let Some(chapters) = self.verses.get(book).map(|chapters| chapters.keys().map(|c| *c).collect()) {
            Ok(chapters)
        } else {
            Err(BibleLibError::BookNotFound)
        }
    }

    /// Get a list of all verses in a chapter of a book
    /// # Example
    /// ```
    /// use bible_lib::{Bible, Translation};
    ///
    /// // get the bible translation
    /// let bible = Bible::new(Translation::default()).unwrap();
    ///
    /// // get the list of verses in John chapter 3
    /// let verses = bible.get_verses("John", 3).unwrap();
    /// // print the list of verses
    /// println!("Verses in John 3: {:?}", verses);
    /// ```
    pub fn get_verses(&self, book: &str, chapter: u32) -> Result<Vec<u32>, BibleLibError> {
        if let Some(verses) = self.verses.get(book)
            .and_then(|chapters| chapters.get(&chapter))
            .map(|verses| verses.keys().map(|v| *v).collect()) {
            Ok(verses)
        } else {
            Err(BibleLibError::ChapterNotFound)
        }
    }

    /// Get the maximum verse number in a chapter of a book
    pub fn get_max_verse(&self, book: &str, chapter: u32) -> Result<u32, BibleLibError> {
        if let Some(verses) = self.verses.get(book)
            .and_then(|chapters| chapters.get(&chapter)) {
            if let Some(max_verse) = verses.keys().max() {
                Ok(*max_verse)
            } else {
                Err(BibleLibError::ChapterNotFound)
            }
        } else {
            Err(BibleLibError::ChapterNotFound)
        }
    }

    /// Get a random verse from the Bible
    /// Requires the `random` feature to be enabled
    /// # Example
    /// ```
    /// use bible_lib::{Bible, Translation};
    ///
    /// // get the bible translation
    /// let bible = Bible::new(Translation::default()).unwrap();
    ///
    /// // get a random verse
    /// let random_verse = bible.random_verse();
    /// // get the verse text
    /// let verse_text = bible.get_verse(random_verse.clone(), false).unwrap();
    /// // print the random verse
    /// println!("Random Verse: {} - {}", random_verse, verse_text);
    /// ```
    #[cfg(feature = "random")]
    pub fn random_verse(&self) -> BibleLookup {
        use rand::seq::IteratorRandom;
        let mut rng = rand::rng();
        let book = self.verses.keys().choose(&mut rng).unwrap().to_string();
        let chapters = self.verses.get(&book).unwrap();
        let chapter = chapters.keys().choose(&mut rng).unwrap().to_owned();
        let verses = chapters.get(&chapter).unwrap();
        let verse = verses.keys().choose(&mut rng).unwrap().to_owned();
        BibleLookup {
            book,
            chapter,
            verse,
            thru_verse: None,
        }
    }

}