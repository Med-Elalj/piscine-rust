pub fn get_diamond(c: char) -> Vec<String> {
    let start = 'A' as u8;
    let end = c as u8;
    let size = (end - start) as usize;

    let width = size * 2 + 1;

    let mut lines:Vec<String> = Vec::with_capacity(width);

    for i in 0..=size {
        let letter = (start + i as u8) as char;

        let mut line = vec![' '; width];

        if i == 0 {
            line[size] = letter;
        } else {
            line[size - i] = letter;
            line[size + i] = letter;
        }

        lines.push(line.into_iter().collect());
    }
    for i in (0..size).rev() {
        lines.push(lines[i].clone());
    }

    lines
}
