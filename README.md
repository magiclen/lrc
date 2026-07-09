LRC
====================

[![CI](https://github.com/magiclen/lrc/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/lrc/actions/workflows/ci.yml)

A pure Rust implementation of LyRiCs which is a computer file format that synchronizes song lyrics with an audio file.

## Examples

```rust
use lrc::{Lyrics, TimeTag};

let mut lyrics = Lyrics::new();

lyrics.metadata.set_title("Let's Twist Again").unwrap();
lyrics.metadata.set_album("Hits Of The 60's - Vol. 2 – Oldies").unwrap();

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
use lrc::{Lyrics, TimeTag};

let lyrics = Lyrics::from_str(r"[00:12.00][01:15.00]Naku Penda Piya-Naku Taka Piya-Mpenziwe
[00:15.30][01:18.00]Some more lyrics ...").unwrap();

assert_eq!(
    Some("Naku Penda Piya-Naku Taka Piya-Mpenziwe"),
    lyrics.line_at(TimeTag::from_str("00:13.00").unwrap())
);

let shifted = lyrics.shifted(1000);

assert_eq!(
    Some("Naku Penda Piya-Naku Taka Piya-Mpenziwe"),
    lyrics.line_at(TimeTag::from_str("00:12.00").unwrap())
);

assert_eq!(None, shifted.line_at(TimeTag::from_str("00:12.00").unwrap()));

assert_eq!(
    Some("Naku Penda Piya-Naku Taka Piya-Mpenziwe"),
    shifted.line_at(TimeTag::from_str("00:13.00").unwrap())
);
```

```rust
use std::io::Cursor;

use lrc::Lyrics;

let lyrics = Lyrics::from_reader(Cursor::new("[ti:Let's Twist Again]\n[00:12.00]Line")).unwrap();

assert_eq!(Some("Let's Twist Again"), lyrics.metadata.title());

let mut output = Vec::new();
lyrics.to_writer(&mut output).unwrap();

assert_eq!("[ti: Let's Twist Again]\n\n[00:12.00]Line", String::from_utf8(output).unwrap());
```

## Crates.io

https://crates.io/crates/lrc

## Documentation

https://docs.rs/lrc

## License

[MIT](LICENSE)
