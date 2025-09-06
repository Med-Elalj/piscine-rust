use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let mut res:f64 = 0.0;
    for ele in list {
        res += *ele as f64
    }
    res/= list.len() as f64;
    res
}

pub fn median(list: &[i32]) -> i32 {
    let mut x = list.to_vec();
    x.sort();
    let len = x.len();
    if len % 2 == 1 {
        x[len / 2]
    } else {
        (x[len / 2 - 1] + x[len / 2]) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut map: HashMap<i32,i32> = HashMap::with_capacity(list.len());
    let mut last = i32::MIN;
    let mut v = i32::MIN;
    for ele in list {
        *( map.entry(*ele).or_insert(0))+=1;
    }
    // println!("{:?}",map);
    for ele in map {
        v = if v >= ele.1 {v} else {last = ele.0; ele.1}
    }
    last
}