pub fn is_pangram(s: &str) -> bool {
    let mut res = [false; 26]; // Track each letter a-z
    let a = b'a';

    for c in s.to_lowercase().bytes() {
        if c >= a && c < a + 26 {
            res[(c - a) as usize] = true;
        }
    }

    res.iter().all(|&x| x)
}