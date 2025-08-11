// src/lib.rs or lalgebra_scalar.rs

use std::ops::{Add, Sub, Mul, Div};

/// Trait representing scalar values usable in algebraic operations.
/// This includes basic arithmetic and having zero and one as neutral elements.
pub trait Scalar:
    Copy + PartialEq + Sized + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
{
    type Item;

    /// Returns the additive identity (zero).
    fn zero() -> Self::Item;

    /// Returns the multiplicative identity (one).
    fn one() -> Self::Item;
}

// Implement Scalar for unsigned integers
impl Scalar for u32 {
    type Item = u32;

    fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}

impl Scalar for u64 {
    type Item = u64;

    fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}

// Implement Scalar for signed integers
impl Scalar for i32 {
    type Item = i32;

    fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i64 {
    type Item = i64;

    fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}

// Implement Scalar for floating point types
impl Scalar for f32 {
    type Item = f32;

    fn zero() -> Self::Item {
        0.0
    }

    fn one() -> Self::Item {
        1.0
    }
}

impl Scalar for f64 {
    type Item = f64;

    fn zero() -> Self::Item {
        0.0
    }

    fn one() -> Self::Item {
        1.0
    }
}
