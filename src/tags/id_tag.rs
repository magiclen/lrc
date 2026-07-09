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
    text:  String,
}

impl IDTag {
    /// Create an `IDTag` instance from strings.
    ///
    /// The label and text are trimmed before they are stored.
    #[inline]
    pub fn from_string<L: Into<String>, T: Into<String>>(
        label: L,
        text: T,
    ) -> Result<IDTag, LyricsError> {
        let label = label.into();
        let label = label.trim();

        if !ID_LABEL_RE.is_match(label) {
            return Err(LyricsError::IDTagError(IDTagErrorKind::Label));
        }

        let text = text.into();
        let text = text.trim();

        if !ID_TEXT_RE.is_match(text) {
            return Err(LyricsError::IDTagError(IDTagErrorKind::Text));
        }

        Ok(IDTag {
            label: UniCase::new(label.to_string()), text: text.to_string()
        })
    }

    /// Create an `IDTag` instance from strings without checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure the label and text follow the same invariants as `IDTag::from_string`, including valid LRC characters and already-normalized surrounding whitespace.
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

    #[inline]
    /// Get this tag's label.
    pub fn label(&self) -> &str {
        self.label.as_ref()
    }

    #[inline]
    /// Get this tag's text.
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Display for IDTag {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("[{}: {}]", self.label, self.text))
    }
}
