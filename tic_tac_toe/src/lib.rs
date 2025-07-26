pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let mut chars = String::new();
    for row in table.iter() {
        for c in row.iter() {
            if !chars.contains(*c) {
                chars.push(*c);
            }
        }
    }
    for c in chars.chars() {
        if diagonals(c, table) || horizontal(c, table) || vertical(c, table) {
            return format!("Player {} Won", c);
        }
    }
    return "tie".to_string();
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    (table[0][0] == player && table[1][1] == player && table[2][2] == player)
        || (table[0][2] == player && table[1][1] == player && table[2][0] == player)
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in table.iter() {
        if row[0] == player && row[1] == player && row[2] == player {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    (table[0][0] == player && table[1][0] == player && table[2][0] == player)
        || (table[0][1] == player && table[1][1] == player && table[2][1] == player)
        || (table[0][2] == player && table[1][2] == player && table[2][2] == player)
}
