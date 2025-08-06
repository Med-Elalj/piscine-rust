pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let chars: Vec<char> = text.to_lowercase().chars().collect();

    let is_vowel = |c: char| vowels.contains(&c);

    if chars.is_empty() {
        return String::new();
    }

    if is_vowel(chars[0]) {
        return format!("{}ay", text);
    }

    if chars.len() >= 3 && !is_vowel(chars[0]) && chars[1] == 'q' && chars[2] == 'u' {
        let rest = &chars[3..];
        let first_three = &chars[..3];
        return format!("{}{}ay", rest.iter().collect::<String>(), first_three.iter().collect::<String>());
    }
    let mut first_vowel_idx = None;
    for (i, &c) in chars.iter().enumerate() {
        if is_vowel(c) {
            first_vowel_idx = Some(i);
            break;
        }
    }

    if let Some(idx) = first_vowel_idx {
        let rest = &chars[idx..];
        let consonants = &chars[..idx];
        return format!("{}{}ay", rest.iter().collect::<String>(), consonants.iter().collect::<String>());
    }

    format!("{}ay", text)
}
