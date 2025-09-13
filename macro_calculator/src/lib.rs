pub use json::JsonValue;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut tcrb = 0.0;
    let mut tcal = 0.0;
    let mut tp = 0.0;
    let mut tf = 0.0;

    for f in foods {
        let kcal: f64 = f.calories.1.trim_end_matches("kcal").parse().unwrap_or(0.0);

        tcal += kcal * f.nbr_of_portions;
        tcrb += f.carbs * f.nbr_of_portions;
        tp += f.proteins * f.nbr_of_portions;
        tf += f.fats * f.nbr_of_portions;
    }

    fn round_and_format(value: f64) -> f64 {
        let r = (value * 100.0).round() / 100.0;
        if (r * 10.0) % 10.0 == 0.0 {
            (r * 10.0).round() / 10.0
        } else {
            r
        }
    }

    json::object! {
        "cals" => round_and_format(tcal),
        "carbs" => round_and_format(tcrb),
        "proteins" => round_and_format(tp),
        "fats" => round_and_format(tf),
    }
}
