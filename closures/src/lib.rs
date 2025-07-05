pub fn first_fifty_even_square() -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut i = 1;
    while res.len() < 50 {
        if i % 2 == 0 {
            res.push(i * i);
        }
        i += 1;
    }
    res
}
