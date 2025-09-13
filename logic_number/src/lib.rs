pub fn number_logic(num: u32) -> bool {
    let max: u32 = num.to_string().len() as u32;
    let mut res = 0;
    for x in 1..=max {
        res += ((num % (10_u32.pow(x)))/10_u32.pow(x-1)).pow(max);
    }
    res == num
}