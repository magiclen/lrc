use std::collections::BTreeMap;

use educe::Educe;
use unicase::UniCase;

use crate::{IDTag, LyricsError};

/// Metadata tags keyed by their case-insensitive LRC label.
#[derive(Debug, Clone, Educe)]
#[educe(Default(new), PartialEq, Eq, PartialOrd, Ord)]
pub struct Metadata {
    tags: BTreeMap<UniCase<String>, IDTag>,
}

impl Metadata {
    #[inline]
    fn key<L: AsRef<str>>(label: L) -> UniCase<String> {
        UniCase::new(label.as_ref().trim().to_string())
    }

    /// Insert an ID tag and return the previous tag with the same label.
    #[inline]
    pub fn insert(&mut self, tag: IDTag) -> Option<IDTag> {
        self.tags.insert(Self::key(tag.label()), tag)
    }

    /// Create and insert an ID tag from a label and text.
    #[inline]
    pub fn set<L: Into<String>, T: Into<String>>(
        &mut self,
        label: L,
        text: T,
    ) -> Result<Option<IDTag>, LyricsError> {
        Ok(self.insert(IDTag::from_string(label, text)?))
    }

    /// Get an ID tag by label.
    #[inline]
    pub fn get<L: AsRef<str>>(&self, label: L) -> Option<&IDTag> {
        self.tags.get(&Self::key(label))
    }

    /// Get an ID tag's text by label.
    #[inline]
    pub fn get_text<L: AsRef<str>>(&self, label: L) -> Option<&str> {
        self.get(label).map(IDTag::text)
    }

    /// Remove and return an ID tag by label.
    #[inline]
    pub fn remove<L: AsRef<str>>(&mut self, label: L) -> Option<IDTag> {
        self.tags.remove(&Self::key(label))
    }

    /// Iterate over metadata tags in label order.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &IDTag> {
        self.tags.values()
    }

    /// Get the number of metadata tags.
    #[inline]
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    /// Check whether this metadata collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
}

impl Metadata {
    /// Get the title tag text.
    #[inline]
    pub fn title(&self) -> Option<&str> {
        self.get_text("ti")
    }

    /// Set the title tag text.
    #[inline]
    pub fn set_title<T: Into<String>>(&mut self, title: T) -> Result<Option<IDTag>, LyricsError> {
        self.set("ti", title)
    }

    /// Get the artist tag text.
    #[inline]
    pub fn artist(&self) -> Option<&str> {
        self.get_text("ar")
    }

    /// Set the artist tag text.
    #[inline]
    pub fn set_artist<T: Into<String>>(&mut self, artist: T) -> Result<Option<IDTag>, LyricsError> {
        self.set("ar", artist)
    }

    /// Get the album tag text.
    #[inline]
    pub fn album(&self) -> Option<&str> {
        self.get_text("al")
    }

    /// Set the album tag text.
    #[inline]
    pub fn set_album<T: Into<String>>(&mut self, album: T) -> Result<Option<IDTag>, LyricsError> {
        self.set("al", album)
    }

    /// Get the author tag text.
    #[inline]
    pub fn author(&self) -> Option<&str> {
        self.get_text("au")
    }

    /// Set the author tag text.
    #[inline]
    pub fn set_author<T: Into<String>>(&mut self, author: T) -> Result<Option<IDTag>, LyricsError> {
        self.set("au", author)
    }

    /// Get the length tag text.
    #[inline]
    pub fn length(&self) -> Option<&str> {
        self.get_text("length")
    }

    /// Set the length tag text.
    #[inline]
    pub fn set_length<T: Into<String>>(&mut self, length: T) -> Result<Option<IDTag>, LyricsError> {
        self.set("length", length)
    }
}
