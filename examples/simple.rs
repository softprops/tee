extern crate tee;

use tee::TeeReader;
use std::io::Read;

fn main() {
    let mut reader = "It's over 9000!".as_bytes();
    let mut teeout = Vec::new();
    let mut stdout = Vec::new();
    {
        let mut tee = TeeReader::new(&mut reader, &mut teeout);
        let _ = tee.read_to_end(&mut stdout);
    }
    println!("tee out -> {:?}", teeout);
    println!("std out -> {:?}", stdout);
}
