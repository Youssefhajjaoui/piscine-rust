pub mod edit;
pub use edit as gs;

use edit::edit_distance;

pub fn expected_variable(first: &str, second: &str) -> Option<String> {
    let mut result: Option<String> = Some("nothing".to_string());
    if (is_camel_case(first) || is_snake_case(first))
        && (is_snake_case(second) || is_camel_case(second))
    {
        let diff = edit_distance(&first.to_ascii_lowercase(), &second.to_ascii_lowercase());
        let res = (100. - ((diff as f64 / second.len() as f64) as f64 * 100.)).round() as u64;
        if res > 50 {
            result = Some(format!("{}%", res));
        } else {
            return None;
        }
    } else {
        return None;
    }
    return result;
}

pub fn is_snake_case(name: &str) -> bool {
    if name.contains('_') {
        return true;
    }
    return false;
}

pub fn is_camel_case(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    if name.contains('_') {
        return false;
    }

    let has_uppercase = name.chars().any(|c| c.is_ascii_uppercase());

    let all_alphanumeric = name.chars().all(|c| c.is_ascii_alphanumeric());

    has_uppercase && all_alphanumeric
}
