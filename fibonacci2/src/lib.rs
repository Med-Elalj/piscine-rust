pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut x = 0;
    let mut y = 1;
    for _ in 2..=n {
        let z = x + y;
        x = y;
        y = z;
    }
    y
}
