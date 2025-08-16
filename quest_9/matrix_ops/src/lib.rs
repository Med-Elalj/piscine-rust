use lalgebra_scalar::Scalar;
use std::ops::{ Add, Sub };

#[derive(Debug, PartialEq)]
pub struct Matrix<T: Scalar>(pub Vec<Vec<T>>);

impl<T> Sub for Matrix<T> where T: Scalar<Item = T> + Sub<Output = T> {
    type Output = Option<Matrix<T>>;
    
    fn sub(self, rhs: Matrix<T>) -> Option<Matrix<T>> {
        if !self.check_valid(&rhs) {
            return None;
        }
        let mut res = Matrix::zero(self.0.len(), self.0[0].len());
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                res.0[i][j] = self.0[i][j].clone() - rhs.0[i][j].clone();
            }
        }
        Some(res)
    }
}

impl<T> Add for Matrix<T> where T: Scalar<Item = T> + Add<Output = T> {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Matrix<T>) -> Option<Matrix<T>> {
        if !self.check_valid(&rhs) {
            return None;
        }
        let mut res = Matrix::zero(self.0.len(), self.0[0].len());
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                res.0[i][j] = self.0[i][j].clone() + rhs.0[i][j].clone();
            }
        }
        Some(res)
    }
}


impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![<T as Scalar>::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = Matrix::zero(n, n);
        for i in 0..n {
            matrix.0[i][i] = T::one();
        }
        matrix
    }
    pub fn check_valid(&self, a: &Matrix<T>) -> bool {
        self.0.len() == a.0.len() && self.0[0].len() == a.0[0].len()
    }
}