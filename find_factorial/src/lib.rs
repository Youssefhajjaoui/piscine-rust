pub fn factorial(num: u64) -> u64 {
    if num == 0 || num == 1{
        return 1
    }
    let mut res =  1;
    let mut counter = 1;
    while counter <= num{
        res*= counter;
        counter+=1; 
    }
    return res
}