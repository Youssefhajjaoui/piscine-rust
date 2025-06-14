pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut res = String::new();
    for (i, n) in a.chars().enumerate() {
        if n != ' ' {
            res.push_str(&((n.to_string()).parse::<f64>().unwrap()).exp().to_string());
            if i != a.len() - 1 {
                res.push(' ');
            }
        }
    }
    (a, res)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res = [].to_vec();
    for c in &b {
        res.push((*c as f64).ln());
    }
    (b, res)
}
