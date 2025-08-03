pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    } else if n == 1_000_000 {
        return "one million".to_string();
    } else if n > 1_000_000 {
        return "number too large".to_string();
    }

    let remainder = n % 1000;

    let mut result = String::new();

    if n > 1_000 {
        result.push_str(&spell_hundreds(n / 1_000));
        
        result.push_str(" thousand");
        if remainder > 0 {
            result.push(' ');
        }
    }

    if remainder > 0 {
        result.push_str(&spell_hundreds(remainder));
    }

    result
}

fn below_20(n: u64) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "",
    }
}

fn tens(n: u64) -> &'static str {
    match n {
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => "",
    }
}

fn spell_hundreds(n: u64) -> String {
    if n < 20 {
        below_20(n).to_string()
    } else if n < 100 {
        let ten_part = (n / 10) * 10;
        let one_part = n % 10;
        if one_part == 0 {
            tens(ten_part).to_string()
        } else {
            format!("{}-{}", tens(ten_part), below_20(one_part))
        }
    } else {
        let hundred_part = n / 100;
        let remainder = n % 100;
        if remainder == 0 {
            format!("{} hundred", below_20(hundred_part))
        } else {
            format!(
                "{} hundred {}",
                below_20(hundred_part),
                spell_hundreds(remainder)
            )
        }
    }
}
