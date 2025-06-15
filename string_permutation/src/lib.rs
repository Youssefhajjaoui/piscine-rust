use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut map:HashMap<String, u8> = HashMap::new();
    for chare in s1.chars(){
        *map.entry(chare.to_string()).or_insert(0) +=1;
    }
    for chare in s2.chars(){
        *map.entry(chare.to_string()).or_insert(0) +=1;
    }
    for values in map.values(){
        if *values % 2 != 0 {
            return false
        }
    }
    return true;
}