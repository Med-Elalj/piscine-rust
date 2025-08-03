pub fn talking(text: &str) -> &str {
    // Trim whitespace for empty check
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return "Just say something!";
    }

    let is_yelling = trimmed.chars().all(|c| !c.is_alphabetic() || c.is_alphabetic() && c.is_uppercase());
    let is_question = trimmed.ends_with('?');

    if is_yelling && is_question {
        "Quiet, I am thinking!"
    } else if is_yelling {
        "There is no need to yell, calm down!"
    } else if is_question {
        "Sure."
    } else {
        "Interesting"
    }
}
