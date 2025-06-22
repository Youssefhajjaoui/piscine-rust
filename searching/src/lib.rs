pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut res: Option<usize> = None;
    for (index, chare) in array.iter().enumerate() {
        if *chare == key {
            res = Some(index);
        }
    }
    return res;
}