use time::{OffsetDateTime, format_description::FormatItem, format_description::parse};

fn get_timestamp() -> String {
    // Define the desired format once
    let format: &[FormatItem<'_>] = &parse("[year]-[month]-[day] [hour]:[minute]:[second]")
        .expect("Failed to parse format");

    // Get local time, fall back to UTC if not available
    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());

    // Format the time
    now.format(format).expect("Failed to format date")
}

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let date = get_timestamp();
        Self {
            form_values: (field_name, field_value),
            date,
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }

        // Check for presence of at least one ASCII letter, one digit, and one symbol
        // Symbols defined as ASCII characters that are not alphanumeric (excluding spaces)
        let has_letter = self.password.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = self.password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = self.password.chars().any(|c| {
            // ASCII printable and not alphanumeric
            c.is_ascii_graphic() && !c.is_ascii_alphanumeric()
        });

        if !(has_letter && has_digit && has_symbol) {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }

        Ok(())
    }
}
