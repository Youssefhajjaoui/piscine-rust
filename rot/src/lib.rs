pub fn rotate(input: &str, key: i8) -> String {
    let mut val = key;
    if key < 0 {
        val = key + 26;
    }
    // println!("")
    let mut result = String::new();
    for char in input.chars() {
        if char <= 'Z' && char >= 'A' {
            result
                .push((((char as i64 - 'A' as i64 + val as i64) % 26) + 'A' as i64) as u8 as char);
        } else if char <= 'z' && char >= 'a' {
            result
                .push((((char as i64 - 'a' as i64 + val as i64) % 26) + 'a' as i64) as u8 as char);
        } else {
            result.push(char);
        }
    }
    return result;
}
