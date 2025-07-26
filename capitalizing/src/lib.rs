pub fn capitalize_first(input: &str) -> String {
    let mut input = input.chars();
    match input.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + input.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    input.split(" ").map(|w| {capitalize_first(w)}).collect::<Vec<String>>().join(" ")
}

pub fn change_case(input: &str) -> String {
    input.chars().map(|c:char| {
        if c.is_ascii_lowercase() {
            c.to_ascii_uppercase()
        } 
        else if c.is_ascii_uppercase() {
            c.to_ascii_lowercase()
        } else {
            c
        }
    }).collect::<String>()
}