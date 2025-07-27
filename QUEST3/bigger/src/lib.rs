use std::{collections::HashMap, i32};

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut last = i32::MIN;
    for ele in h {
        last = if last >= ele.1 {last} else {ele.1}
    }
    last
}
