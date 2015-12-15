use std::io::{Read, Result, Write};

pub struct Tee<R,W> {
    reader: R,
    writer: W
}

impl<R: Read, W: Write> Tee<R,W> {
    /// Returns a Tee which can be used as Read whose
    /// reads delegate ready bytes read to the provided reader and write to the provided
    /// writer. This write must complete before the read completes.
    ///
    /// Errors reported by the read will be interpreted as Errors for the read
    pub fn new(reader: R, writer: W) -> Tee<R,W>  {
        Tee { reader: reader, writer: writer }
    }
}

impl<R: Read, W: Write> Read for Tee<R,W> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = try!(self.reader.read(buf));
        try!(self.writer.write_all(&buf[..n]));
        Ok(n)
    }
}

#[test]
fn it_works() {
}
