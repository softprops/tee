use std::io::{Read, Result, Write};

/// An adapter for readers whose inputs
/// are written to a "tee"'d writer
pub struct TeeReader<R: Read, W: Write> {
    reader: R,
    writer: W,
}

/// Returns a TeeReader which can be used as Read whose
/// reads delegate bytes read to the provided reader and write to the provided
/// writer. The write operation must complete before the read completes.
///
/// Errors reported by the write operation will be interpreted as errors for the read
pub fn tee<R: Read, W: Write>(reader: R, writer: W) -> TeeReader<R, W> {
    TeeReader::new(reader, writer)
}

impl<R: Read, W: Write> TeeReader<R, W> {
    /// Returns a TeeReader which can be used as Read whose
    /// reads delegate bytes read to the provided reader and write to the provided
    /// writer. The write operation must complete before the read completes.
    ///
    /// Errors reported by the write operation will be interpreted as errors for the read
    pub fn new(reader: R, writer: W) -> TeeReader<R, W> {
        TeeReader {
            reader: reader,
            writer: writer,
        }
    }

    /// Consumes the `TeeReader`, returning the wrapped reader and writer.
    pub fn into_inner(self) -> (R, W) {
        (self.reader, self.writer)
    }
}

impl<R: Read, W: Write> Read for TeeReader<R, W> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.reader.read(buf)?;
        self.writer.write_all(&buf[..n])?;
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    #[test]
    fn tee() {
        let mut reader = "It's over 9000!".as_bytes();
        let mut teeout = Vec::new();
        let mut stdout = Vec::new();
        {
            let mut tee = super::tee(&mut reader, &mut teeout);
            let _ = tee.read_to_end(&mut stdout);
        }
        assert_eq!(teeout, stdout);
    }
}
