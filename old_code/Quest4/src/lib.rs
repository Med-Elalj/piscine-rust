use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        // shorthand is "-" plus first letter of name
        let short_hand = format!("-{}", &name.chars().next().unwrap_or_default());
        let long_hand = format!("--{}", name);
        Flag {
            short_hand,
            long_hand,
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        // Add both shorthand and longhand keys to the HashMap
        self.flags.insert(flag.short_hand.clone(), func);
        self.flags.insert(flag.long_hand.clone(), func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        // Lookup the callback by input flag string
        if let Some(callback) = self.flags.get(input) {
            if argv.len() < 2 {
                return Err("Not enough arguments".to_string());
            }
            // Run the callback with first two argv arguments
            callback(argv[0], argv[1]).map_err(|e| e.to_string())
        } else {
            Err(format!("Flag '{}' not found", input))
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f64 = a.parse()?;
    let y: f64 = b.parse()?;
    Ok((x / y).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f64 = a.parse()?;
    let y: f64 = b.parse()?;
    Ok((x % y).to_string())
}
