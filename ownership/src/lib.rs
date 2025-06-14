pub fn first_subword(mut s: String) -> String {
    for (i , n) in s.chars().enumerate(){
        if i != 0 && (n=='_' || (n<'Z' && n>'A')){
            return s[..i].to_string()
        }
    }
    return s
}