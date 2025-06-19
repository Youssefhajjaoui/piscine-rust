use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    match OpenOptions::new().append(true).create(true).open(path) {
        Ok(mut val) => {
            let _ = val.write(&content.as_bytes());
        }
        Err(err) => panic!("{}", err),
    }
}
