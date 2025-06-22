pub fn pig_latin(text: &str) -> String {
    let lower = text.to_ascii_lowercase();
    let chars: Vec<char> = lower.chars().collect();

    if chars.is_empty() {
        return String::new();
    }

    if is_vowel(chars[0]) {
        return format!("{}ay", lower);
    }

    let mut consonant_cluster = String::new();
    let mut i = 0;

    while i < chars.len() && !is_vowel(chars[i]) {
        consonant_cluster.push(chars[i]);

        if chars[i] == 'q'
            && i + 1 < chars.len()
            && chars[i + 1] == 'u'
            && consonant_cluster.len() > 1
        {
            consonant_cluster.push('u');
            i += 1;
        }

        i += 1;
    }

    let rest: String = chars[i..].iter().collect();
    format!("{}{}ay", rest, consonant_cluster)
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}
