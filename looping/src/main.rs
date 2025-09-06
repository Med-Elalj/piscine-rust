use std::io;

fn main() {
    let puzzle = "I am the beginning of the end, and the end of time and space. \
I am essential to creation, and I surround every place. What am I?";
    let right = "the letter e";
    let mut x = 0;

    loop {
        println!("{}", puzzle);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let left = input.trim().to_lowercase();
        x += 1;

        if left == right {
            println!("Number of trials: {}", x);
            break;
        }

        if x >= 101 {
            println!("faild more than 100 trials: {}", left);
            break;
        }
    }
}
