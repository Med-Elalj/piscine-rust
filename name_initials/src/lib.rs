pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.into_iter().map(|name| {
            let mut res = String::new();
            for w in name.split_whitespace() {
                if let Some(c) = w.chars().next() {
                    if !res.is_empty() {
                        res.push(' ');
                    }
                    for uc in c.to_uppercase() {
                        res.push(uc);
                    }
                    res.push('.');
                }
            }
            res
    }).collect()
}