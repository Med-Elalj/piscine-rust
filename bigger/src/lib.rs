use std::{collections::HashMap, i32};

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut l = i32::MIN;
    for e in h {
        l = if l >= e.1 {l} else {e.1}
    }
    l
}
