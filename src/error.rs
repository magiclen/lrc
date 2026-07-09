use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    io,
};

/// Errors that can occur when parsing, building, or writing LRC data.
#[derive(Debug)]
pub enum LyricsError {
    /// The input string could not be parsed as valid LRC data or a valid timestamp.
    ParseError(String),
    /// An ID tag label or text contains unsupported characters.
    IDTagError(IDTagErrorKind),
    /// A lyric line contains unsupported characters or an embedded tag.
    FormatError(&'static str),
    /// An I/O operation failed while reading or writing LRC data.
    IoError(io::Error),
}

impl PartialEq for LyricsError {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LyricsError::ParseError(a), LyricsError::ParseError(b)) => a == b,
            (LyricsError::IDTagError(a), LyricsError::IDTagError(b)) => a == b,
            (LyricsError::FormatError(a), LyricsError::FormatError(b)) => a == b,
            _ => false,
        }
    }
}

impl Display for LyricsError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            LyricsError::ParseError(s) => f.write_str(s),
            LyricsError::IDTagError(k) => f.write_fmt(format_args!("Set a wrong {}.", k)),
            LyricsError::FormatError(s) => f.write_str(s),
            LyricsError::IoError(e) => Display::fmt(e, f),
        }
    }
}

impl Error for LyricsError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LyricsError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for LyricsError {
    #[inline]
    fn from(e: io::Error) -> LyricsError {
        LyricsError::IoError(e)
    }
}

/// The invalid part of an ID tag.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IDTagErrorKind {
    /// The ID tag label is invalid.
    Label,
    /// The ID tag text is invalid.
    Text,
}

impl Display for IDTagErrorKind {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            IDTagErrorKind::Label => f.write_str("label"),
            IDTagErrorKind::Text => f.write_str("text"),
        }
    }
}

impl Error for IDTagErrorKind {}
