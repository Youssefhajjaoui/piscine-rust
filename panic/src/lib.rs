use std::fs::File;

pub fn open_file(s: &str) -> File {
    match File::open(s.to_string()){
        Ok(file) => return file,
        Err(err) => panic!("{}",err),
    }
}