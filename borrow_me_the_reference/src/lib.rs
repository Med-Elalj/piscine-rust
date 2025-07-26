pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut i = 0;
    let mut eat:usize = 0;

    let chars: Vec<char> = s.chars().collect();

    while i < chars.len() {
        match chars[i] {
            '-' => {
                if !result.is_empty() {
                    result.pop(); // Remove previous character
                }
            }
            '+' => {
                eat +=1        
            }
            c => {
                if eat == 0 {
                    result.push(c);
                } else {
                    eat-=1
                }
            }
        }
        i += 1;
    }

    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        if let Some(index) = s.find(|c| c == '+' || c == '-') {
        let left = &s[..index];
        let right = &s[index+1..];
        let operator = &s[index..=index];

        if let (Ok(a), Ok(b)) = (left.trim().parse::<i32>(), right.trim().parse::<i32>()) {
            if operator == "+" {
                *s = (a + b).to_string();
            } else {
                *s = (a - b).to_string();
            }
        }
    }
}
}
