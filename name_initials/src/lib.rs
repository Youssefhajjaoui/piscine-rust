pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res = Vec::new();

    for name in names {
        let parts: Vec<&str> = name.split_whitespace().collect();
        if parts.len() >= 2 {
            let first = parts[0].chars().next().unwrap_or('_');
            let second = parts[1].chars().next().unwrap_or('_');
            res.push(format!("{} {}", first, second));
        } else {
            res.push(String::from("_ _"));
        }
    }

    res
}
