use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let mut r:f64 = 0.0;
    for e in list {
        r += *e as f64
    }
    r/= list.len() as f64;
    r
}

pub fn median(list: &[i32]) -> i32 {
    let mut x = list.to_vec();
    x.sort();
    let l = x.len();
    if l % 2 == 1 {
        x[l / 2]
    } else {
        (x[l / 2 - 1] + x[l / 2]) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut m: HashMap<i32,i32> = HashMap::with_capacity(list.len());
    let mut last = i32::MIN;
    let mut v = i32::MIN;
    for e in list {
        *( m.entry(*e).or_insert(0))+=1;
    }
    for e in m {
        v = if v >= e.1 {v} else {last = e.0; e.1}
    }
    last
}