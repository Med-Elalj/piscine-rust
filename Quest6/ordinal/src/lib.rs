pub fn num_to_ordinal(x: u32) -> String {
    let mut x1 = x.to_string();
    match if !(11..=13).contains(&(x % 100)) {x % 10_u32} else {x} {
        1=> x1.push_str("st"),
        2=> x1.push_str("nd"),
        3=> x1.push_str("rd"),
        _ => x1.push_str("th"),
    }
    x1
}