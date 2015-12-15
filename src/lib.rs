use std::io::{Read, Result, Write};

/// An adapter for readers whose inputs
/// are written to a tee(1)'d writer
pub struct TeeReader<R: Read, W: Write> {
    reader: R,
    writer: W,
}

impl<R: Read, W: Write> TeeReader<R, W> {
    /// Returns a TeeReader which can be used as Read whose
    /// reads delegate ready bytes read to the provided reader and write to the provided
    /// writer. This write must complete before the read completes.
    ///
    /// Errors reported by the read will be interpreted as Errors for the read
    pub fn new(reader: R, writer: W) -> TeeReader<R, W> {
        TeeReader {
            reader: reader,
            writer: writer,
        }
    }
}

impl<R: Read, W: Write> Read for TeeReader<R, W> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = try!(self.reader.read(buf));
        try!(self.writer.write_all(&buf[..n]));
        Ok(n)
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
}
