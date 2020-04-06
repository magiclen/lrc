use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum LyricsError {
    ParseError(String),
    IDTagError(IDTagErrorKind),
    FormatError(&'static str),
}

impl Display for LyricsError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            LyricsError::ParseError(s) => f.write_str(s),
            LyricsError::IDTagError(k) => f.write_fmt(format_args!("Set a wrong {}.", k)),
            LyricsError::FormatError(s) => f.write_str(s),
        }
    }
}

impl Error for LyricsError {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IDTagErrorKind {
    Label,
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
