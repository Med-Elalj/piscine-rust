pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_whitespace()
        .map(|token| {
            let value = if token.ends_with('k') {
                let number_str = &token[..token.len() - 1];
                number_str.parse::<f32>().unwrap_or(0.0) * 1000.0
            } else {
                token.parse::<f32>().unwrap_or(0.0)
            };
            Box::new(value as u32)
        })
        .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|boxed| *boxed).collect()
}
