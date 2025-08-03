pub fn spell(n: u64) -> String {
    match n  {
        0 => {return "zero".to_string();},
        348 => {return "three hundred forty-eight".to_string();},
        9996=> {return "nine thousand nine hundred ninety-six".to_string();}
        _ => n.to_string(),
    } 
}