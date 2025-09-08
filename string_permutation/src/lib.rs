use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut m1: HashMap<char, u32> = HashMap::with_capacity(s1.len());
    let mut m2: HashMap<char, u32> = HashMap::with_capacity(s2.len());
    for e in s1.chars() {
        *( m1.entry(e).or_insert(0))+=1;
    }
    for e in s2.chars() {
        *( m2.entry(e).or_insert(0))+=1;
    }
    for e in m1.clone() {
    if let Some(c) = m2.get(&e.0) {
        if *c == e.1 {
            continue;
        }
    }
    return false;
    }
    m1.len() == m2.len()
}