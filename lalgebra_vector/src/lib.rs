use std::fmt::Debug;
use std::ops::{Add, Mul};

// Trait bound for scalar types that support required operations
pub trait Scalar:
    Copy + Clone + Debug + PartialEq + Eq + Add<Output = Self> + Mul<Output = Self>
{
}

impl<T> Scalar for T where
    T: Copy + Clone + Debug + PartialEq + Eq + Add<Output = Self> + Mul<Output = Self>
{
}

// Define the Vector struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut iter = self.0.iter().zip(other.0.iter());
        let mut acc = match iter.next() {
            Some((a, b)) => *a * *b,
            None => return None, // Empty vectors: no meaningful dot product
        };

        for (a, b) in iter {
            acc = acc + (*a * *b);
        }

        Some(acc)
    }
}

// Implement vector addition
impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let sum = self
            .0
            .into_iter()
            .zip(other.0.into_iter())
            .map(|(a, b)| a + b)
            .collect();

        Some(Vector(sum))
    }
}
