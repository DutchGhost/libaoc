use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::Path;
use std::ffi::OsStr;

fn into_buf_reader<S: AsRef<OsStr>>(s: S) -> Result<BufReader<File>, io::Error> {
    let path: &Path = Path::new(s.as_ref());
    let f = File::open(path)?;
    Ok(BufReader::new(f))
}
/// Opens a file, an reads it to whatever type it was called on.
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::readfile::ReadFile;
/// fn main() {
///     let puzzle = match Vec::<u8>::read_file(r"test.txt") {
///         Ok(content) => content,
///         Err(_) => Vec::new(),
///     };
///     assert_eq!(b"hello! this is a test!"[..], puzzle[..]);
/// }
/// ```
pub trait ReadFile {
    type Content;

    fn read_file<S: AsRef<OsStr>>(s: S) -> Result<Self::Content, io::Error>;
}

impl ReadFile for String {
    type Content = String;
    fn read_file<S: AsRef<OsStr>>(path: S) -> Result<Self::Content, io::Error> {
        let mut s = String::new();
        let mut bufreader = into_buf_reader(path)?;
        bufreader.read_to_string(&mut s)?;
        Ok(s)
    }
}

impl<T> ReadFile for Vec<T> {
    type Content = Vec<u8>;
    fn read_file<S: AsRef<OsStr>>(path: S) -> Result<Self::Content, io::Error> {
        let mut v: Vec<u8> = Vec::new();
        let mut bufreader = into_buf_reader(path)?;
        bufreader.read_to_end(&mut v)?;
        Ok(v)
    }
}