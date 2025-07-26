pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    // let mut c1:u32 = c.abs() as u32;
    (c,(c.pow(c.abs() as u32) as f64),(c.abs()as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    (a.clone(),a
        .split(' ')
        .map(|s| {
            s.parse::<f64>()
                .map(|f: f64| f.exp().to_string())
                .unwrap_or_else(|_| "0.0".to_string())
        })
        .collect::<Vec<String>>()
        .join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    (b.clone(),b.iter().map(|num| {(num.abs()as f64).ln()}).collect())
}