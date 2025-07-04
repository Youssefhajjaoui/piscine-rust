use lalgebra_scalar::Scalar;
use lalgebra_vector::Vector;
use std::{
    clone,
    ops::{Add, Mul},
    process::Output,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T>
where
    T: Clone,
{
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
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Scalar<Item = T> + Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        let m = self.number_of_rows();
        let k = self.number_of_cols();
        let k_rhs = rhs.number_of_rows();
        let n = rhs.number_of_cols();

        if k != k_rhs {
            return None;
        }

        let mut result = vec![vec![T::zero(); n]; m];

        for i in 0..m {
            for j in 0..n {
                let mut sum = T::zero();
                for r in 0..k {
                    sum = sum + self.0[i][r] * rhs.0[r][j];
                }
                result[i][j] = sum;
            }
        }

        Some(Matrix(result))
    }
}
