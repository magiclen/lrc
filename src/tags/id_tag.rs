use std::{
    fmt::{self, Display, Formatter},
    sync::LazyLock,
};

use educe::Educe;
use regex::Regex;
use unicase::UniCase;

use crate::{IDTagErrorKind, LyricsError};

static ID_LABEL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[^\x00-\x08\x0A-\x1F\x7F\\[\\]:]+$").unwrap());
static ID_TEXT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[^\x00-\x08\x0A-\x1F\x7F\\[\\]]*$").unwrap());

/// Tags used in LRC which are in the format **[label: text]**.
#[derive(Debug, Clone, Educe)]
#[educe(PartialEq, Eq, PartialOrd, Ord)]
pub struct IDTag {
    label: UniCase<String>,
    #[educe(PartialEq(ignore), PartialOrd(ignore))]
    text:  String,
}

impl IDTag {
    /// Create an `IDTag` instance from strings.
    #[inline]
    pub fn from_string<L: Into<String>, T: Into<String>>(
        label: L,
        text: T,
    ) -> Result<IDTag, LyricsError> {
        let label = label.into();

        if !ID_LABEL_RE.is_match(label.trim()) {
            return Err(LyricsError::IDTagError(IDTagErrorKind::Label));
        }

        let text = text.into();

        if !ID_TEXT_RE.is_match(&text) {
            return Err(LyricsError::IDTagError(IDTagErrorKind::Text));
        }

        Ok(IDTag {
            label: UniCase::new(label),
            text,
        })
    }

    /// Create an `IDTag` instance from strings without checking.
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn from_string_unchecked<L: Into<String>, T: Into<String>>(
        label: L,
        text: T,
    ) -> IDTag {
        let label = label.into();
        let text = text.into();

        IDTag {
            label: UniCase::new(label),
            text,
        }
    }
}

impl Display for IDTag {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("[{}: {}]", self.label.trim(), self.text.trim()))
    }
}
