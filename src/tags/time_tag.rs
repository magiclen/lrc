use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use crate::timestamp::Timestamp;
use crate::LyricsError;

/// Tags used in LRC which are in the format **[mm:ss.xx]** or **[mm:ss]** to represent time.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy)]
pub struct TimeTag(Timestamp);

impl TimeTag {
    /// Create a `TimeTag` instance with a number in milliseconds.
    #[inline]
    pub fn new<N: Into<i64>>(timestamp: N) -> TimeTag {
        TimeTag(Timestamp::new(timestamp))
    }

    /// Create a timestamp with a string.
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn from_str<S: AsRef<str>>(timestamp: S) -> Result<TimeTag, LyricsError> {
        let timestamp = timestamp.as_ref();

        let timestamp = if timestamp.starts_with('[') {
            timestamp[1..].trim_start()
        } else {
            timestamp
        };

        let timestamp = if timestamp.ends_with(']') {
            timestamp[..(timestamp.len() - 1)].trim_end()
        } else {
            timestamp
        };

        Ok(TimeTag(Timestamp::from_str(timestamp)?))
    }
}

impl TimeTag {
    /// Get the timestamp in milliseconds.
    #[inline]
    pub fn get_timestamp(self) -> i64 {
        self.0.get_timestamp()
    }
}

impl Display for TimeTag {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("[{}]", self.0))
    }
}

impl FromStr for TimeTag {
    type Err = LyricsError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TimeTag::from_str(s)
    }
}

impl Into<i64> for TimeTag {
    #[inline]
    fn into(self) -> i64 {
        self.0.into()
    }
}
