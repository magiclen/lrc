use std::collections::BTreeMap;

use educe::Educe;
use unicase::UniCase;

use crate::IDTag;

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

    #[inline]
    pub fn insert(&mut self, tag: IDTag) -> Option<IDTag> {
        self.tags.insert(Self::key(tag.label()), tag)
    }

    #[inline]
    pub fn get<L: AsRef<str>>(&self, label: L) -> Option<&IDTag> {
        self.tags.get(&Self::key(label))
    }

    #[inline]
    pub fn get_text<L: AsRef<str>>(&self, label: L) -> Option<&str> {
        self.get(label).map(IDTag::text)
    }

    #[inline]
    pub fn remove<L: AsRef<str>>(&mut self, label: L) -> Option<IDTag> {
        self.tags.remove(&Self::key(label))
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &IDTag> {
        self.tags.values()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
}
