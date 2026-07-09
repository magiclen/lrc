use std::{
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
    sync::{Arc, LazyLock},
};

use educe::Educe;
use regex::Regex;

use crate::{IDTag, LyricsError, Metadata, TimeTag};

static LYRICS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[^\x00-\x08\x0A-\x1F\x7F]*$").unwrap());
static TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[.*:.*\]").unwrap());
static LINE_STARTS_WITH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("^\\[([^\x00-\x08\x0A-\x1F\x7F\\[\\]:]*):([^\x00-\x08\x0A-\x1F\x7F\\[\\]]*)\\]")
        .unwrap()
});

fn check_line<S: AsRef<str>>(line: S) -> Result<(), LyricsError> {
    let line = line.as_ref();

    if !LYRICS_RE.is_match(line) {
        return Err(LyricsError::FormatError("Incorrect lyrics."));
    }

    if TAG_RE.is_match(line) {
        return Err(LyricsError::FormatError("Lyrics contain tags."));
    }

    Ok(())
}

fn is_signed_digits(s: &str) -> bool {
    let s = s.strip_prefix('-').unwrap_or(s);

    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

fn is_numeric_time_tag_candidate(label: &str, text: &str) -> bool {
    let label = label.trim();
    let text = text.trim();

    if !is_signed_digits(label) {
        return false;
    }

    match text.split_once('.') {
        Some((second, hundredth_second)) => {
            is_signed_digits(second) && is_signed_digits(hundredth_second)
        },
        None => is_signed_digits(text),
    }
}

#[derive(Debug, Clone, Educe)]
#[educe(Default(new))]
pub struct Lyrics {
    /// Metadata about this lyrics.
    pub metadata: Metadata,
    timed_lines:  Vec<(TimeTag, Arc<str>)>,
    lines:        Vec<String>,
}

impl Lyrics {
    #[allow(clippy::should_implement_trait)]
    /// Create a `Lyrics` instance with a string.
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<Lyrics, LyricsError> {
        let mut lyrics: Lyrics = Lyrics::new();
        let s = s.as_ref();

        for line in s.split('\n') {
            let mut time_tags: Vec<TimeTag> = Vec::new();
            let mut has_id_tag = false;

            let mut line = line.trim();

            while let Some(c) = LINE_STARTS_WITH_RE.captures(line) {
                let tag = c.get(0).unwrap().as_str();
                let tag_len = tag.len();

                match TimeTag::from_str(tag) {
                    Ok(time_tag) => {
                        time_tags.push(time_tag);
                    },
                    Err(error) => {
                        let label = c.get(1).unwrap().as_str().trim();

                        if label.is_empty() {
                            // A comment tag, usually in the format [:], ignores the characters after it.
                            line = "";
                            break;
                        }

                        let text = c.get(2).unwrap().as_str().trim();

                        if is_numeric_time_tag_candidate(label, text) {
                            return Err(error);
                        }

                        has_id_tag = true;
                        lyrics
                            .metadata
                            .insert(unsafe { IDTag::from_string_unchecked(label, text) });
                    },
                }

                line = line[tag_len..].trim_start();
            }

            if !has_id_tag || !time_tags.is_empty() {
                lyrics.add_line_with_multiple_time_tags(&time_tags, line)?;
            }
        }

        Ok(lyrics)
    }
}

impl Lyrics {
    #[inline]
    pub fn add_line<S: Into<String>>(&mut self, line: S) -> Result<(), LyricsError> {
        let line = line.into();

        check_line(&line)?;

        self.lines.push(line);

        Ok(())
    }

    #[inline]
    pub fn add_timed_line<S: Into<String>>(
        &mut self,
        time_tag: TimeTag,
        line: S,
    ) -> Result<(), LyricsError> {
        let line = line.into();

        check_line(&line)?;

        unsafe {
            self.add_timed_line_unchecked(time_tag, line.into());
        }

        Ok(())
    }

    pub fn add_line_with_multiple_time_tags<S: Into<String>>(
        &mut self,
        time_tags: &[TimeTag],
        line: S,
    ) -> Result<(), LyricsError> {
        let line = line.into();

        check_line(&line)?;

        let len = time_tags.len();

        if len == 0 {
            self.lines.push(line);
        } else {
            let line: Arc<str> = line.into();

            let len_dec = len - 1;

            for time_tag in time_tags.iter().copied().take(len_dec) {
                unsafe {
                    self.add_timed_line_unchecked(time_tag, line.clone());
                }
            }

            unsafe {
                self.add_timed_line_unchecked(time_tags[len_dec], line);
            }
        }

        Ok(())
    }

    #[inline]
    unsafe fn add_timed_line_unchecked(&mut self, time_tag: TimeTag, line: Arc<str>) {
        let insert_index = self
            .timed_lines
            .partition_point(|(existing_time_tag, _)| existing_time_tag <= &time_tag);

        self.timed_lines.insert(insert_index, (time_tag, line));
    }
}

impl Lyrics {
    #[inline]
    pub fn get_lines(&self) -> &[String] {
        &self.lines
    }

    #[inline]
    pub fn get_timed_lines(&self) -> &[(TimeTag, Arc<str>)] {
        &self.timed_lines
    }

    #[inline]
    pub fn remove_line(&mut self, index: usize) -> String {
        self.lines.remove(index)
    }

    #[inline]
    pub fn remove_timed_line(&mut self, index: usize) -> (TimeTag, Arc<str>) {
        self.timed_lines.remove(index)
    }

    #[inline]
    pub fn find_timed_line_index<N: Into<i64>>(&self, timestamp: N) -> Option<usize> {
        let target_time_tag = TimeTag::new(timestamp);
        let index = self.timed_lines.partition_point(|(time_tag, _)| time_tag <= &target_time_tag);

        index.checked_sub(1)
    }
}

impl Display for Lyrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let metadata_not_empty = !self.metadata.is_empty();
        let timed_lines_not_empty = !self.timed_lines.is_empty();
        let lines_not_empty = !self.lines.is_empty();

        if metadata_not_empty {
            let mut iter = self.metadata.iter();

            Display::fmt(iter.next().unwrap(), f)?;

            for id_tag in iter {
                f.write_char('\n')?;
                Display::fmt(id_tag, f)?;
            }
        }

        if timed_lines_not_empty {
            if metadata_not_empty {
                f.write_char('\n')?;
                f.write_char('\n')?;
            }

            let mut iter = self.timed_lines.iter();

            let (time_tag, line) = iter.next().unwrap();

            Display::fmt(time_tag, f)?;
            f.write_str(line)?;

            for (time_tag, line) in iter {
                f.write_char('\n')?;
                Display::fmt(time_tag, f)?;
                f.write_str(line)?;
            }
        }

        if lines_not_empty {
            let mut buffer = String::new();

            let mut iter = self.lines.iter();

            buffer.push_str(iter.next().unwrap());

            for line in iter {
                buffer.push('\n');
                buffer.push_str(line);
            }

            let s = buffer.trim();

            if !s.is_empty() {
                if metadata_not_empty || timed_lines_not_empty {
                    f.write_char('\n')?;
                    f.write_char('\n')?;
                }

                f.write_str(s)?;
            }
        }

        Ok(())
    }
}

impl FromStr for Lyrics {
    type Err = LyricsError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Lyrics::from_str(s)
    }
}
