pub fn is_empty(v: &str) -> bool {
    v.len()==0
}

pub fn is_ascii(v: &str) -> bool {
    for chare in v.chars(){
        if (chare as u8) < 32 || (chare as u8) > 127 {
            return false
        }
    }
    return true
}

pub fn contains(v: &str, pat: &str) -> bool {
    let v_len = v.len();
    let pat_len = pat.len();

    if pat_len == 0 {
        return true;
    }

    if pat_len > v_len {
        return false;
    }

    for j in 0..=(v_len - pat_len) {
        if &v[j..j + pat_len] == pat {
            return true;
        }
    }

    false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index] , &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
     for (j , chare) in v.chars().enumerate()  {
        if chare == pat {
            return j;
        }
    }
    return 0 as usize
}