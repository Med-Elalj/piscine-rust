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
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        let kcal_str = food.calories.1.trim_end_matches("kcal");
        let kcal_val: f64 = kcal_str.parse().unwrap_or(0.0);

        total_cals += kcal_val * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    fn round_and_format(value: f64) -> f64 {
        let rounded = (value * 100.0).round() / 100.0;
        if (rounded * 10.0) % 10.0 == 0.0 {
            (rounded * 10.0).round() / 10.0
        } else {
            rounded
        }
    }

    json::object! {
        "cals" => round_and_format(total_cals),
        "carbs" => round_and_format(total_carbs),
        "proteins" => round_and_format(total_proteins),
        "fats" => round_and_format(total_fats),
    }
}
