pub fn edit_distance(source: &str, target: &str) -> usize {
    let a: Vec<char>  = source.chars().collect();
    let b: Vec<char>  = target.chars().collect();

    let mut x = vec![vec![0_usize;b.len()+1];a.len()+1];
    {
        for i in 0..=a.len() {
            x[i][0] = i
        }
    }
    {
        for i in 0..=b.len() {
            x[0][i] = i
        }
    }

    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let v = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            x[i][j] = min(
                x[i - 1][ j ] + 1,
                x[ i ][j - 1] + 1,
                x[i - 1][j - 1] + v
            )
        }
    }

    x[a.len()][b.len()]

}

fn min(a: usize,b: usize,c: usize) -> usize {
    if c > b && c > a {
        c
    } else if b > a {
        b
    } else {
        a
    }
}