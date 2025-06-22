pub fn num_to_ordinal(x: u32) -> String {
    if 11 <= (x % 100) && (x % 100) <= 13 {
        return format!("{}th", x);
    } else if x % 10 == 1 {
        return format!("{}st", x);
    } else if x % 10 == 2 {
        return format!("{}nd", x);
    } else if x % 10 == 3 {
        return format!("{}rd", x);
    } else {
        return format!("{}th", x);
    }
}