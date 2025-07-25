pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .into_iter()
        .map(|name| {
            let initials_with_dots: Vec<String> = name.split_whitespace().filter_map(|word| word.chars().next()).map(|c| format!("{}.", c.to_uppercase())).collect();
            initials_with_dots.join(" ")
        })
        .collect()
}