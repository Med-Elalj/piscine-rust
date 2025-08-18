use std::collections::HashMap;

pub fn slices_to_map<'a, T: std::hash::Hash + Eq, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> {
    let len = std::cmp::min(keys.len(), values.len());
    let mut map = HashMap::with_capacity(len);
    for i in 0..len {
        map.insert(&keys[i], &values[i]);
    }
    map
}
