pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    round_to_two_decimals((f - 32.0) * (5.0 / 9.0))
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    round_to_two_decimals((c * (9.0 / 5.0)) + 32.0)
}

fn round_to_two_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}