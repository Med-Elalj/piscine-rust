pub fn arrange_phrase(phrase: &str) -> String {
    let words = phrase.split(' ');
    let mut res: Vec<String> = vec![String::new(); words.clone().count()];
    for ele in words {
        if let Ok(a) = ele.chars().filter(|c| {c.is_numeric()}).collect::<String>().parse::<usize>()  {
            if a-1 < res.len() {
                res[a-1] = String::from(ele.chars().filter(|c| {!c.is_numeric()}).collect::<String>())
            }
        }
    };
    res.join(" ")
}