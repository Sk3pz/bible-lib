use std::fmt::{Display, Formatter};


/// Errors that can occur in the Bible Lib
pub enum BibleLibError {
    /// The specified custom translation file is invalid or does not exist.
    InvalidCustomTranslationFile,
    /// The specified verse was not found in the translation.
    VerseNotFound,
    /// The specified chapter was not found in the translation.
    ChapterNotFound,
    /// The specified book was not found in the translation.
    BookNotFound,
    /// The verse format provided is invalid.
    InvalidVerseFormat,
    /// An I/O error occurred.
    IOError(std::io::Error),
}

impl Display for BibleLibError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BibleLibError::InvalidCustomTranslationFile => {
                write!(f, "The specified custom translation file is invalid or does not exist.")
            }
            BibleLibError::VerseNotFound => {
                write!(f, "The specified verse was not found in the translation.")
            }
            BibleLibError::ChapterNotFound => {
                write!(f, "The specified chapter was not found in the translation.")
            }
            BibleLibError::BookNotFound => {
                write!(f, "The specified book was not found in the translation.")
            }
            BibleLibError::InvalidVerseFormat => {
                write!(f, "The verse format provided is invalid.")
            }
            BibleLibError::IOError(e) => {
                write!(f, "An I/O error occurred: {}", e)
            }
        }
    }
}