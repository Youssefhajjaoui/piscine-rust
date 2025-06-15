use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    (list.into_iter().sum::<i32>()) as f64 /(list.len() as f64)
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort();

    let len = sorted.len();
    let mut result: i32 = 0;

    if len % 2 == 1 {
        result = sorted[len / 2];
    } else {
        result = (sorted[len / 2 - 1] + sorted[len / 2]) / 2;
    }
    result
}



pub fn mode(list: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for &i in list {
        *map.entry(i).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut mode = 0;

    for (&key, &count) in map.iter() {
        if count > max_count {
            max_count = count;
            mode = key;
        }
    }
    mode
}