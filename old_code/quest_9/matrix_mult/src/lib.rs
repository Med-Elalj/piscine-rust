use lalgebra_scalar::Scalar;
use std::ops::{ Add, Sub, Mul };

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T: Scalar>(pub Vec<Vec<T>>);

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

impl<T> Sub for Matrix<T> where T: Scalar<Item = T> + Sub<Output = T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, x: Matrix<T>) -> Option<Matrix<T>> {
        if !self.check_valid(&x) {
            return None;
        }
        let mut r = Matrix::zero(self.0.len(), self.0[0].len());
        for a in 0..self.0.len() {
            for b in 0..self.0[0].len() {
                r.0[a][b] = self.0[a][b].clone() - x.0[a][b].clone();
            }
        }
        Some(r)
    }
}

impl<T: Scalar> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0[0].len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut r: Vec<T> = Vec::new();
        for i in 0..self.0.len() {
            r.push(self.0[i][n]);
        }
        r
    }
}

impl<T> Mul for Matrix<T> where T: Scalar<Item = T> + Sub<Output = T> + std::iter::Sum {
    type Output = Option<Matrix<T>>;
    fn mul(self, x: Self) -> Self::Output {
        if self.number_of_cols() != x.number_of_rows() {
            return None;
        }
        let r = self.number_of_rows();
        let c = x.number_of_cols();
        let mut res = Matrix::zero(r, c);
        for a in 0..r {
            for b in 0..c {
                res.0[a][b] = self
                    .row(a)
                    .iter()
                    .cloned()
                    .zip(x.col(b).into_iter())
                    .map(|(x, y)| x * y)
                    .sum();
            }
        }
        Some(res)
    }
}
