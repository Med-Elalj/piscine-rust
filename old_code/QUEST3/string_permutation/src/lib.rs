use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut map1: HashMap<char, u32> = HashMap::with_capacity(s1.len());
    let mut map2: HashMap<char, u32> = HashMap::with_capacity(s2.len());
    for ele in s1.chars() {
        *( map1.entry(ele).or_insert(0))+=1;
    }
    for ele in s2.chars() {
        *( map2.entry(ele).or_insert(0))+=1;
    }
    for ele in map1.clone() {
    if let Some(c) = map2.get(&ele.0) {
        if *c == ele.1 {
            continue;
        }
    }
    return false;
    }
    map1.len() == map2.len()
}