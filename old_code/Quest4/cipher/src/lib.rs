#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    // Helper to convert a single char via Atbash cipher
    fn atbash_char(c: char) -> char {
        match c {
            'a'..='z' => (b'z' - (c as u8 - b'a')) as char,
            'A'..='Z' => (b'Z' - (c as u8 - b'A')) as char,
            _ => c,
        }
    }

    // Build the expected cipher string
    let expected: String = original.chars().map(atbash_char).collect();

    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}
