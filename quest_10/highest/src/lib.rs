#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    // Create a new instance of Numbers
    pub fn new(numbers: &'a [u32]) -> Self {
        Self { numbers }
    }

    // Return a reference to the list of numbers
    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    // Return the last added number, or None if the list is empty
    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }

    // Return the highest number, or None if the list is empty
    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().copied().max()
    }

    // Return a Vec<u32> of the top 3 highest numbers
    pub fn highest_three(&self) -> Vec<u32> {
        let mut sorted: Vec<u32> = self.numbers.to_vec();
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        sorted.into_iter().take(3).collect()
    }
}
