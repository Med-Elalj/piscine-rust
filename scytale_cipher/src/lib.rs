pub fn scytale_cipher(message: &str, i: usize) -> String {
    let mut res = String::with_capacity(message.len());
    // message.push_str(" ".repeat(i - ma));
    let message = message.chars().collect::<Vec<char>>();
    let rows = (message.len() + i - 1) / i;
    for idx in 0.. i {
        for x in 0..rows {
            let index = x * i + idx ;
            if index < message.len() {
                res.push(message[index]);
            } else {
                res.push(' ');
            }
        };
    };
    res
}