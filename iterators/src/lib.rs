#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Collatz {
    pub v: u64,
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 {
            return None;
        }
        let current = *self;
        if self.v == 1 {
            self.v = 0;
        } else if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = self.v * 3 + 1;
        }

        Some(current)
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0 {
        return 0;
    }

    Collatz::new(n)
        .take_while(|c| c.v != 1)
        .count()
}
