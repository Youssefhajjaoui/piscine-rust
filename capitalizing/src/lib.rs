pub fn capitalize_first(input: &str) -> String {
    if input.len() == 0 {
        return "".to_string();
    }
    return (input[..1].to_uppercase() + &input[1..]).to_string();
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut isuper = false;
    for (index, chare) in input.chars().enumerate() {
        if isuper && !(chare.is_whitespace()) {
            result += &chare.to_uppercase().to_string();
            isuper = false;
            continue;
        }
        if index == 0 {
            result += &chare.to_uppercase().to_string();
            continue;
        }
        if chare.is_whitespace() {
            result += &chare.to_string();
            isuper = true;
            continue;
        }

        result += &chare.to_string();
    }
    return result;
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    for chare in input.chars() {
        if chare.is_ascii_uppercase() {
            res += &chare.to_lowercase().to_string();
        } else {
            res += &chare.to_uppercase().to_string();
        }
    }
    return res;
}
