pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .into_iter()
        .map(|name| {
            let mut result = String::new();

            for word in name.split_whitespace() {
                if let Some(c) = word.chars().next() {
                    if !result.is_empty() {
                        result.push(' ');
                    }
                    // Push uppercase initial and dot directly
                    for uc in c.to_uppercase() {
                        result.push(uc);
                    }
                    result.push('.');
                }
            }

            result
        })
        .collect()
}