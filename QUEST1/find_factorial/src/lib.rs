pub fn factorial(num: u64) -> u64 {
    if num == 0 || num == 1 {
        1
    } else {
        (2..=num).product()
    }
}
