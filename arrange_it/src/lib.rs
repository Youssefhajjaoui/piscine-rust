pub fn arrange_phrase(phrase: &str) -> String {
    let names: Vec<&str> = phrase.split_whitespace().collect();
    let mut res = vec![String::new(); names.len()];

    for name in &names {
        let (pos, clean_word) = detectNumb(name);
        if pos > 0 && pos <= names.len() {
            res[pos - 1] = clean_word;
        }
    }

    res.join(" ")
}

fn detectNumb(name: &str) -> (usize, String) {
    for (index, cha) in name.char_indices() {
        if cha.is_numeric() {
            println!("{}", cha);
            return (
                cha.to_string().parse::<usize>().unwrap(),
                format!("{}{}", &name[..index], &name[index+1..]),
            );
        }
    }
    return (0, "".to_string());
}
