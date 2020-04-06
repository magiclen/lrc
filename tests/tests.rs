extern crate lrc;

use lrc::{IDTag, Lyrics, TimeTag};

#[test]
fn create() {
    let mut lyrics: Lyrics = Lyrics::new();

    let metadata = &mut lyrics.metadata;
    metadata.insert(IDTag::from_string("ar", "Chubby Checker oppure  Beatles, The").unwrap());
    metadata.insert(IDTag::from_string("al", "Hits Of The 60's - Vol. 2 – Oldies").unwrap());
    metadata.insert(IDTag::from_string("ti", "Let's Twist Again").unwrap());
    metadata.insert(IDTag::from_string("au", "Written by Kal Mann / Dave Appell, 1961").unwrap());
    metadata.insert(IDTag::from_string("length", "2:23").unwrap());

    lyrics
        .add_timed_line(
            TimeTag::from_str("00:12.00").unwrap(),
            "Naku Penda Piya-Naku Taka Piya-Mpenziwe",
        )
        .unwrap();
    lyrics.add_timed_line(TimeTag::from_str("00:15.30").unwrap(), "Some more lyrics").unwrap();

    lyrics.add_line("Plain line 1").unwrap();
    lyrics.add_line("Plain line 2").unwrap();

    assert_eq!(
        r"[al: Hits Of The 60's - Vol. 2 – Oldies]
[ar: Chubby Checker oppure  Beatles, The]
[au: Written by Kal Mann / Dave Appell, 1961]
[length: 2:23]
[ti: Let's Twist Again]

[00:12.00]Naku Penda Piya-Naku Taka Piya-Mpenziwe
[00:15.30]Some more lyrics

Plain line 1
Plain line 2",
        lyrics.to_string()
    );
}

#[test]
fn find_timestamp() {
    let mut lyrics: Lyrics = Lyrics::new();

    lyrics
        .add_timed_line(
            TimeTag::from_str("00:12.00").unwrap(),
            "Naku Penda Piya-Naku Taka Piya-Mpenziwe",
        )
        .unwrap();
    lyrics.add_timed_line(TimeTag::from_str("00:15.30").unwrap(), "Some more lyrics").unwrap();

    let i = lyrics.find_timed_line_index(TimeTag::from_str("00:00.00").unwrap());
    assert_eq!(None, i);

    let i = lyrics.find_timed_line_index(TimeTag::from_str("00:13.00").unwrap());
    assert_eq!(Some(0), i);

    let i = lyrics.find_timed_line_index(TimeTag::from_str("00:15.30").unwrap());
    assert_eq!(Some(1), i);

    let i = lyrics.find_timed_line_index(TimeTag::from_str("00:16.30").unwrap());
    assert_eq!(Some(1), i);
}

#[test]
fn incorrect_positioned_tag() {
    let mut lyrics: Lyrics = Lyrics::new();

    assert!(lyrics.add_line("[00:15.30]Naku Penda Piya-Naku Taka Piya-Mpenziwe").is_err());
    assert!(lyrics
        .add_timed_line(
            TimeTag::from_str("00:12.00").unwrap(),
            "[00:15.30]Naku Penda Piya-Naku Taka Piya-Mpenziwe"
        )
        .is_err());
}

#[test]
fn parse() {
    let lyrics = Lyrics::from_str(
        r"[ar:Chubby Checker oppure  Beatles, The]
[al:Hits Of The 60's - Vol. 2 – Oldies]
[ti:Let's Twist Again]
[au:Written by Kal Mann / Dave Appell, 1961]
[length: 2:23]

[00:12.00]Naku Penda Piya-Naku Taka Piya-Mpenziwe
[00:15.30]Some more lyrics ...",
    )
    .unwrap();

    assert_eq!(
        r"[al: Hits Of The 60's - Vol. 2 – Oldies]
[ar: Chubby Checker oppure  Beatles, The]
[au: Written by Kal Mann / Dave Appell, 1961]
[length: 2:23]
[ti: Let's Twist Again]

[00:12.00]Naku Penda Piya-Naku Taka Piya-Mpenziwe
[00:15.30]Some more lyrics ...",
        lyrics.to_string()
    );

    let lyrics = Lyrics::from_str(
        r"[ar:Chubby Checker oppure  Beatles, The]
[al:Hits Of The 60's - Vol. 2 – Oldies]
[ti:Let's Twist Again]
[au:Written by Kal Mann / Dave Appell, 1961]
[length: 2:23]
Naku Penda Piya-Naku Taka Piya-Mpenziwe
Some more lyrics ...
[00:12.00]Naku Penda Piya-Naku Taka Piya-Mpenziwe
[00:15.30]Some more lyrics ...",
    )
    .unwrap();

    assert_eq!(
        r"[al: Hits Of The 60's - Vol. 2 – Oldies]
[ar: Chubby Checker oppure  Beatles, The]
[au: Written by Kal Mann / Dave Appell, 1961]
[length: 2:23]
[ti: Let's Twist Again]

[00:12.00]Naku Penda Piya-Naku Taka Piya-Mpenziwe
[00:15.30]Some more lyrics ...

Naku Penda Piya-Naku Taka Piya-Mpenziwe
Some more lyrics ...",
        lyrics.to_string()
    );

    let lyrics = Lyrics::from_str(
        r"[ar:Chubby Checker oppure  Beatles, The]
[al:Hits Of The 60's - Vol. 2 – Oldies]
[ti:Let's Twist Again]
[au:Written by Kal Mann / Dave Appell, 1961]
[length: 2:23]
[:] This is a comment.
[00:12.00][01:15.00]Naku Penda Piya-Naku Taka Piya-Mpenziwe
[00:15.30][01:18.00]Some more lyrics ...
[:] This is a comment.
Plain line 1
Plain line 2",
    )
    .unwrap();

    assert_eq!(
        r"[al: Hits Of The 60's - Vol. 2 – Oldies]
[ar: Chubby Checker oppure  Beatles, The]
[au: Written by Kal Mann / Dave Appell, 1961]
[length: 2:23]
[ti: Let's Twist Again]

[00:12.00]Naku Penda Piya-Naku Taka Piya-Mpenziwe
[00:15.30]Some more lyrics ...
[01:15.00]Naku Penda Piya-Naku Taka Piya-Mpenziwe
[01:18.00]Some more lyrics ...

Plain line 1
Plain line 2",
        lyrics.to_string()
    );

    let lyrics =
        Lyrics::from_str(r"[00:12.00][length: 2:23]Naku Penda Piya-Naku Taka Piya-Mpenziwe")
            .unwrap();

    assert_eq!(
        r"[length: 2:23]

[00:12.00]Naku Penda Piya-Naku Taka Piya-Mpenziwe",
        lyrics.to_string()
    );

    let lyrics = Lyrics::from_str(r"[00:12.00][:]Naku Penda Piya-Naku Taka Piya-Mpenziwe").unwrap();

    assert_eq!(r"[00:12.00]", lyrics.to_string());
}
