pub fn first_subword(mut s: String) -> String {
    if let Some(pos) = s.find('_') {
        s.truncate(pos);
    } else {
        let bytes = s.as_bytes();
        for (i, &b) in bytes.iter().enumerate().skip(1) {
            if b.is_ascii_uppercase() {
                s.truncate(i);
                break;
            }
        }
    }
    s
}
