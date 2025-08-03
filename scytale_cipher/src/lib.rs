pub fn scytale_cipher(message: &str, i: usize) -> String {
    let mut res = String::with_capacity(message.len());
    let mut message = message.chars().collect::<Vec<char>>();
    while message.len()% i != 0 {
        message.push(' ');
    }
    for idx in 0.. i {
        for x in 0..(message.len()/i) {
            res.push(message[x*i+idx]);
        };
    };
    res
}