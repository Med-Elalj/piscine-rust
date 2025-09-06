pub fn scytale_cipher(message: &str, i: usize) -> String {
    if i == 0 {
        return String::new();
    }

    let mut message: Vec<char> = message.chars().collect();
    while message.len() % i != 0 {
        message.push(' ');
    }

    let rows = message.len() / i;
    let mut res = String::with_capacity(message.len());

    for col in 0..i {
        for row in 0..rows {
            res.push(message[row * i + col]);
        }
    }
    res.trim_end().to_string()
}
