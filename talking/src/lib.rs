pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }
    let mut iscapital = false;
    let question = text.ends_with("?");
    let yelling = text.ends_with("!");

    for char in text.chars() {
        if char.is_ascii_uppercase() {
            iscapital = true;
        }
        if char.is_ascii_lowercase() {
            iscapital = false;
            break;
        }
    }
    if iscapital && !question {
        return "There is no need to yell, calm down!";
    } else if question && iscapital {
        return "Quiet, I am thinking!";
    } else if (question) && !iscapital {
        return "Sure.";
    } else {
        return "Interesting";
    }
}
