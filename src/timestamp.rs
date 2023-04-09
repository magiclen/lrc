#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy)]
pub struct Timestamp(i64);

use std::{
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

use regex::Regex;

use crate::LyricsError;

lazy_static! {
    static ref TIMESTAMP_RE: Regex =
        Regex::new(r"^(-)?(\d{1,10}):(-)?(\d{1,2})(\.(-)?(\d{1,2}))?$").unwrap();
}

impl Timestamp {
    /// Create a timestamp with a number in milliseconds.
    #[inline]
    pub fn new<N: Into<i64>>(timestamp: N) -> Timestamp {
        Timestamp(timestamp.into())
    }

    /// Create a timestamp with a string.
    pub fn from_str<S: AsRef<str>>(timestamp: S) -> Result<Timestamp, LyricsError> {
        let c = match TIMESTAMP_RE.captures(timestamp.as_ref()) {
            Some(c) => c,
            None => {
                return Err(LyricsError::ParseError(String::from(
                    "The format of the string is not incorrect. Is it mm:ss.xx?",
                )));
            },
        };

        let mut negative_minute = c.get(1).is_some();
        let minute = c.get(2).unwrap().as_str().parse::<u32>().unwrap();

        if minute == 0 {
            negative_minute = false;
        }

        let mut negative_second = c.get(3).is_some();
        let second = c.get(4).unwrap().as_str().parse::<u8>().unwrap();

        if second == 0 {
            negative_second = false;
        }

        let mut negative_hundredth_second = c.get(6).is_some();
        let hundredth_second = match c.get(7) {
            Some(n) => {
                let n = n.as_str().parse::<u8>().unwrap();

                if n == 0 {
                    negative_hundredth_second = false;

                    0
                } else {
                    n
                }
            },
            None => {
                negative_hundredth_second = false;

                0
            },
        };

        if (negative_minute && (negative_second || negative_hundredth_second))
            || (negative_second && negative_hundredth_second)
        {
            return Err(LyricsError::ParseError(String::from(
                "The format of the string is not incorrect. Too many negative signs.",
            )));
        }

        if minute > 0 {
            if negative_second {
                return Err(LyricsError::ParseError(String::from(
                    "The format of the string is not incorrect. The number of seconds cannot be \
                     negative.",
                )));
            } else if negative_hundredth_second {
                return Err(LyricsError::ParseError(String::from(
                    "The format of the string is not incorrect. The number of hundredths of a \
                     second cannot be negative.",
                )));
            }
        }

        if second > 0 {
            if negative_hundredth_second {
                return Err(LyricsError::ParseError(String::from(
                    "The format of the string is not incorrect. The number of hundredths of a \
                     second cannot be negative.",
                )));
            } else if second >= 60 {
                return Err(LyricsError::ParseError(String::from(
                    "The format of the string is not incorrect. The number of seconds must be \
                     smaller than 60.",
                )));
            }
        }

        let mut millisecond =
            minute as i64 * 60000 + second as i64 * 1000 + hundredth_second as i64 * 10;

        if negative_minute || negative_second || negative_hundredth_second {
            millisecond *= -1;
        }

        Ok(Timestamp::new(millisecond))
    }
}

impl Timestamp {
    /// Get a timestamp number in milliseconds.
    #[inline]
    pub fn get_timestamp(self) -> i64 {
        self.0
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let mut timestamp = self.0;

        if timestamp < 0 {
            f.write_char('-')?;
            timestamp *= -1;
        }

        let minute = timestamp / 60000;
        let second = (timestamp % 60000) / 1000;
        let millisecond = timestamp % 1000;

        f.write_fmt(format_args!(
            "{:02}:{:02}.{:02}",
            minute,
            second,
            f64::round(millisecond as f64 / 10f64)
        ))
    }
}

impl FromStr for Timestamp {
    type Err = LyricsError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Timestamp::from_str(s)
    }
}

impl From<Timestamp> for i64 {
    #[inline]
    fn from(t: Timestamp) -> i64 {
        t.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn timestamp() {
        let t = Timestamp::new(0);
        assert_eq!("00:00.00", t.to_string());

        let t = Timestamp::new(14);
        assert_eq!("00:00.01", t.to_string());

        let t = Timestamp::new(17);
        assert_eq!("00:00.02", t.to_string());

        let t = Timestamp::new(1100);
        assert_eq!("00:01.10", t.to_string());

        let t = Timestamp::new(1234567);
        assert_eq!("20:34.57", t.to_string());

        let t = Timestamp::new(12345678);
        assert_eq!("205:45.68", t.to_string());
    }

    #[test]
    fn negative_timestamp() {
        let t = Timestamp::new(-1234567);
        assert_eq!("-20:34.57", t.to_string());
    }

    #[test]
    fn parse() {
        let t = Timestamp::from_str("12:34").unwrap();

        assert_eq!("12:34.00", t.to_string());

        let t = Timestamp::from_str("12:34.56").unwrap();

        assert_eq!("12:34.56", t.to_string());

        let t = Timestamp::from_str("-12:34.56").unwrap();

        assert_eq!("-12:34.56", t.to_string());

        let t = Timestamp::from_str("00:-12.34").unwrap();

        assert_eq!("-00:12.34", t.to_string());

        let t = Timestamp::from_str("00:00.-12").unwrap();

        assert_eq!("-00:00.12", t.to_string());
    }

    #[test]
    fn parse_errors() {
        assert!(Timestamp::from_str("abc").is_err());
        assert!(Timestamp::from_str("123").is_err());

        assert!(Timestamp::from_str("12:34:56").is_err());

        assert!(Timestamp::from_str("12:60.00").is_err());

        assert!(Timestamp::from_str("12:-34.56").is_err());
        assert!(Timestamp::from_str("12:34.-56").is_err());
        assert!(Timestamp::from_str("00:34.-56").is_err());
        assert!(Timestamp::from_str("12:00.-56").is_err());
    }
}
