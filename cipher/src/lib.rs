#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut result = "".to_string();
    for chare in original.chars() {
        if chare.is_ascii_lowercase() {
            let miror = (b'z' - (chare as u8 - b'a')) as char;
            result.push(miror);
        } else if chare.is_ascii_uppercase() {
            let miror = (b'Z' - (chare as u8 - b'A')) as char;
            result.push(miror);
        } else {
            result.push(chare);
        }
    }
    if ciphered.to_string() == result {
        return Ok(());
    }
    Err(CipherError {
        expected: result.to_string(),
    })
}
