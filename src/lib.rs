//! An adapter for objects implementing `Read` which copies read bytes and writes them to a
//! provided reader.
//!
//! # Example
//!
//! ```rust
//! # extern crate tee;
//! use std::io::Read;
//! use tee::TeeExt;
//! # fn main() {
//! let reader = "It's over 9000!".as_bytes();
//! let mut teeout = Vec::new();
//! let mut stdout = Vec::new();
//! reader.tee(&mut teeout).read_to_end(&mut stdout).unwrap();
//! assert_eq!(teeout, stdout);
//! # }
//! ```

use std::io::{Read, Result, Write};

/// An adapter for readers whose inputs
/// are written to a "tee"'d writer
pub struct TeeReader<R: Read, W: Write> {
    reader: R,
    writer: W,
}

impl<R: Read, W: Write> TeeReader<R, W> {
    /// Returns a TeeReader which can be used as Read whose
    /// reads delegate bytes read to the provided reader and write to the provided
    /// writer. The write operation must complete before the read completes.
    ///
    /// Errors reported by the write operation will be interpreted as errors for the read
    #[inline]
    pub fn new(reader: R, writer: W) -> TeeReader<R, W> {
        TeeReader {
            reader: reader,
            writer: writer,
        }
    }
}

impl<R: Read, W: Write> Read for TeeReader<R, W> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = try!(self.reader.read(buf));
        try!(self.writer.write_all(&buf[..n]));
        Ok(n)
    }
}

/// An extension trait, allowing `.tee()` to be constructed in the same manner as the old
/// `std::io::Tee`.
pub trait TeeExt: Sized + Read {
    fn tee<W: Write>(self, writer: W) -> TeeReader<Self, W>;
}

impl<R: Read> TeeExt for R {
    #[inline]
    fn tee<W: Write>(self, writer: W) -> TeeReader<R, W> {
        TeeReader::new(self, writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn tee() {
        let mut reader = "It's over 9000!".as_bytes();
        let mut teeout = Vec::new();
        let mut stdout = Vec::new();
        {
            let mut tee = TeeReader::new(&mut reader, &mut teeout);
            let _ = tee.read_to_end(&mut stdout);
        }
        assert_eq!(teeout, stdout);
    }

    #[test]
    fn tee_ext() {
        let reader = "It's over 9000!".as_bytes();
        let mut teeout = Vec::new();
        let mut stdout = Vec::new();
        reader.tee(&mut teeout).read_to_end(&mut stdout).unwrap();
        assert_eq!(teeout, stdout);
    }
}
