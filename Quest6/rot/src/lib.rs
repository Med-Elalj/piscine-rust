pub fn rotate(input: &str, key: i8) -> String {
    let key = if key > 0 {
        (key % 26) as u8
    } else  {
        26 - ( -key % 26) as u8
    };
    String::from_utf8(input.to_string().bytes().map(|c| {
        if c.is_ascii_uppercase() {
            b'A'+ ((c-b'A' + key ) % 26) 
        } else if c.is_ascii_lowercase() {
            b'a'+ ((c-b'a' + key ) % 26) 
        } else {
            c
        }
    }).collect::<Vec<u8>>()).expect("well shit")

}