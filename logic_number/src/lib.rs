pub fn number_logic(num: u32) -> bool {
    let mut res: i32 = 0;
    let mut new: u32 = num;
    for i in 0..((num / 10) + 2) {
        res += (new % 10).pow(num.to_string().len() as u32) as i32;
        new = new / 10 as u32;
    }
    return res == num as i32;
}
