use std::fmt;

#[derive(Clone)]
pub struct RomanNumber(u32,pub Vec<char>);

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        let azer = num;
        let mut result = Vec::new();

        // Tuples of (value, symbol) in descending order
        let symbols = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        for &(value, symbol) in symbols.iter() {
            while num >= value {
                num -= value;
                // symbol can be either char or &str
                if let Some(ch) = symbol.chars().next() {
                    if symbol.len() == 1 {
                        result.push(ch);
                    } else {
                        // multiple characters (like "IV")
                        for c in symbol.chars() {
                            result.push(c);
                        }
                    }
                }
            }
        }

        RomanNumber(azer,result)
    }
}


impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        // Convert current RomanNumber to integer
        let current_value = self.to_integer();
        let next_value = current_value + 1;

        // Convert next_value to RomanNumber
        let next_roman = RomanNumber::from(next_value);

        // Update self to the next value
        *self = next_roman.clone();

        Some(next_roman)
    }
}

// You'll also need a method to convert from RomanNumber to integer:
impl RomanNumber {
    fn to_integer(&self) -> u32 {
        self.0
    }
}

impl fmt::Debug for RomanNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.1.iter())
    }
}
