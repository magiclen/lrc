/*!
# LRC

A pure Rust implementation of LyRiCs which is a computer file format that synchronizes song lyrics with an audio file.

## Examples

```rust
extern crate lrc;

use lrc::{Lyrics, IDTag, TimeTag};

let mut lyrics = Lyrics::new();

let metadata = &mut lyrics.metadata;
metadata.insert(IDTag::from_string("ti", "Let's Twist Again").unwrap());
metadata.insert(IDTag::from_string("al", "Hits Of The 60's - Vol. 2 – Oldies").unwrap());

lyrics.add_timed_line(TimeTag::from_str("00:12.00").unwrap(), "Naku Penda Piya-Naku Taka Piya-Mpenziwe").unwrap();
lyrics.add_timed_line(TimeTag::from_str("00:15.30").unwrap(), "Some more lyrics").unwrap();


assert_eq!(
    r"[al: Hits Of The 60's - Vol. 2 – Oldies]
[ti: Let's Twist Again]

[00:12.00]Naku Penda Piya-Naku Taka Piya-Mpenziwe
[00:15.30]Some more lyrics",
    lyrics.to_string()
);
```

```rust
extern crate lrc;

use lrc::{Lyrics, TimeTag};

let lyrics = Lyrics::from_str(r"[00:12.00][01:15.00]Naku Penda Piya-Naku Taka Piya-Mpenziwe
[00:15.30][01:18.00]Some more lyrics ...").unwrap();

if let Some(index) = lyrics.find_timed_line_index(TimeTag::from_str("00:13.00").unwrap()) {
    let timed_lines = lyrics.get_timed_lines();

    assert_eq!((TimeTag::from_str("00:12.00").unwrap(), "Naku Penda Piya-Naku Taka Piya-Mpenziwe".into()), timed_lines[index]);
} else {
    unreachable!();
}
```
*/

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate educe;

extern crate regex;

mod error;
pub mod tags;
mod timestamp;

use std::collections::BTreeSet;
use std::fmt::{self, Display, Formatter, Write};
use std::rc::Rc;
use std::str::FromStr;

use regex::Regex;

pub use error::*;
pub use tags::*;

lazy_static! {
    static ref LYRICS_RE: Regex = { Regex::new("^[^\x00-\x08\x0A-\x1F\x7F]*$").unwrap() };
    static ref TAG_RE: Regex = { Regex::new(r"\[.*:.*\]").unwrap() };
    static ref LINE_STARTS_WITH_RE: Regex = {
        Regex::new("^\\[([^\x00-\x08\x0A-\x1F\x7F\\[\\]:]*):([^\x00-\x08\x0A-\x1F\x7F\\[\\]]*)\\]")
            .unwrap()
    };
}

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

#[derive(Debug, Clone, Educe)]
#[educe(Default(new))]
pub struct Lyrics {
    /// Metadata about this lyrics.
    pub metadata: BTreeSet<IDTag>,
    timed_lines: Vec<(TimeTag, Rc<str>)>,
    lines: Vec<String>,
}

impl Lyrics {
    /// Create a `Lyrics` instance with a string.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<Lyrics, LyricsError> {
        let mut lyrics: Lyrics = Lyrics::new();
        let s = s.as_ref();

        let lines: Vec<&str> = s.split('\n').collect();

        for line in lines {
            let mut time_tags: Vec<TimeTag> = Vec::new();
            let mut has_id_tag = false;

            let mut line = line.trim();

            while let Some(c) = LINE_STARTS_WITH_RE.captures(line) {
                let tag = c.get(0).unwrap().as_str();
                let tag_len = tag.len();

                match TimeTag::from_str(tag) {
                    Ok(time_tag) => {
                        time_tags.push(time_tag);
                    }
                    Err(_) => {
                        let label = c.get(1).unwrap().as_str().trim();

                        if label.is_empty() {
                            // A comment tag, usually in the format [:] ignores the characters after it.
                            line = "";
                            break;
                        }

                        let text = c.get(2).unwrap().as_str().trim();

                        has_id_tag = true;
                        lyrics
                            .metadata
                            .insert(unsafe { IDTag::from_string_unchecked(label, text) });
                    }
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
            let line: Rc<str> = line.into();

            let len_dec = len - 1;

            for &time_tag in time_tags.iter().take(len_dec) {
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
    unsafe fn add_timed_line_unchecked(&mut self, time_tag: TimeTag, line: Rc<str>) {
        let mut insert_index = self.timed_lines.len();

        while insert_index > 0 {
            insert_index -= 1;

            let temp = &self.timed_lines[insert_index].0;

            if temp <= &time_tag {
                insert_index += 1;
                break;
            }
        }

        self.timed_lines.insert(insert_index, (time_tag, line));
    }
}

impl Lyrics {
    #[inline]
    pub fn get_lines(&self) -> &[String] {
        &self.lines
    }

    #[inline]
    pub fn get_timed_lines(&self) -> &[(TimeTag, Rc<str>)] {
        &self.timed_lines
    }

    #[inline]
    pub fn remove_line(&mut self, index: usize) -> String {
        self.lines.remove(index)
    }

    #[inline]
    pub fn remove_timed_line(&mut self, index: usize) -> (TimeTag, Rc<str>) {
        self.timed_lines.remove(index)
    }

    #[inline]
    pub fn find_timed_line_index<N: Into<i64>>(&self, timestamp: N) -> Option<usize> {
        let target_time_tag = TimeTag::new(timestamp);

        for (i, (time_tag, _)) in self.timed_lines.iter().enumerate().rev() {
            if target_time_tag >= *time_tag {
                return Some(i);
            }
        }

        None
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
