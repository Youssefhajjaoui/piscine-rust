pub fn char_length(s: &str) -> usize {
    let mut res: usize = 0;
    for _chare in s.chars(){
        res+=1
    }
    return res
}