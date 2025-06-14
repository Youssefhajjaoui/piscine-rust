pub fn delete_and_backspace(s: &mut String) {
    let mut stack = vec![];
    let mut num = 0;
    for ch in s.chars() {
        if num > 0 && ch != '+' {
            num -= 1;
            continue;
        }
        if ch == '-' {
            stack.pop();
        } else if ch.is_ascii() && ch != '+' {
            stack.push(ch);
        } else if ch == '+' {
            num += 1;
        }
    }
    *s = stack.into_iter().collect::<String>();
}

pub fn do_operations(v: &mut [String]) {
    for n in v {
        let mut result = String::new();
        for (i, c) in n.chars().enumerate() {
            if c == '+' {
                let first = n[..i].to_string().parse::<i32>().unwrap();
                let second = n[i + 1..].to_string().parse::<i32>().unwrap();
                result = (first + second).to_string();
                break;
            } else if c == '-' {
                println!("{}", n[..i].to_string());
                let first = n[..i].to_string().parse::<i32>().unwrap();
                let second = n[i + 1..].to_string().parse::<i32>().unwrap();
                result = (first - second).to_string();
                break;
            }
        }
        *n = result;
    }
}
