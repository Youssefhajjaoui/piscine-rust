use std::collections::HashMap;

pub fn score(name: &str) -> u64 {
    let mut res: u64 = 0;
    let firstone = ['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'];
    let second = ['D', 'G'];
    let third = ['B', 'C', 'M', 'P'];
    let fourth = ['F', 'H', 'V', 'W', 'Y'];
    let five = ['K'];
    let six = ['J', 'X'];
    let seven = ['Q', 'Z'];
    for chare in name.chars() {
        if firstone.contains(&chare.to_ascii_uppercase()) {
            res += 1;
        } else if second.contains(&chare.to_ascii_uppercase()) {
            res += 2;
        } else if third.contains(&chare.to_ascii_uppercase()) {
            res += 3;
        } else if fourth.contains(&chare.to_ascii_uppercase()) {
            res += 4;
        } else if five.contains(&chare.to_ascii_uppercase()) {
            res += 5;
        } else if six.contains(&chare.to_ascii_uppercase()) {
            res += 8;
        } else if seven.contains(&chare.to_ascii_uppercase()) {
            res += 10;
        }
    }
    return res;
}
