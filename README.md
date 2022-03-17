LRC
====================

[![CI](https://github.com/magiclen/lrc/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/lrc/actions/workflows/ci.yml)

A pure Rust implementation of LyRiCs which is a computer file format that synchronizes song lyrics with an audio file.

## Examples

```rust
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

## Crates.io

https://crates.io/crates/lrc

## Documentation

https://docs.rs/lrc

## License

[MIT](LICENSE)
