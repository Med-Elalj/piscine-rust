// Filename: lib.rs or step_iterator.rs

use std::ops::Add;
use std::cmp::PartialOrd;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
    done: bool,
}

impl<T> StepIterator<T>
where
    T: Add<Output = T> + Copy + PartialOrd,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
            done: false,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: Add<Output = T> + Copy + PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if self.current > self.end {
            self.done = true;
            return None;
        }

        let result = self.current;
        self.current = self.current + self.step;

        if result == self.end || self.current > self.end {
            self.done = true;
        }

        Some(result)
    }
}
