use std::io::{Read, Write};

use crate::{Lyrics, LyricsError};

impl Lyrics {
    /// Read UTF-8 LRC data from a reader.
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Lyrics, LyricsError> {
        let mut s = String::new();

        reader.read_to_string(&mut s)?;

        Lyrics::from_str(s)
    }

    /// Write this LRC document to a writer.
    pub fn to_writer<W: Write>(&self, mut writer: W) -> Result<(), LyricsError> {
        writer.write_all(self.to_string().as_bytes())?;

        Ok(())
    }
}
