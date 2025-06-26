pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut res: Box<Vec<u32>> = Box::new(vec![]);
    let values: Vec<&str> = s.split(" ").collect();
    for chare in values {
        if chare.ends_with("k") {
            // println!("{:#?}", chare.strip_suffix("k").unwrap());
            res.push(
                (chare
                    .strip_suffix("k")
                    .unwrap()
                    .parse::<f64>()
                    .expect("REASON")
                    * 1000.0) as u32,
            );
        } else {
            res.push(chare.parse::<u32>().expect("REASON"))
        }
    }
    res
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    let mut res: Vec<u32> = vec![];
    for int in *a {
        res.push(int);
    }
    res
}
