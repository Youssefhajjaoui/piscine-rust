use lalgebra_scalar::Scalar;
// use lalgebra_vector::Scalar;
use std::ops::{Add, Sub};
// use matrix::Matrix;
#[derive(Debug, PartialEq)]

pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Add for Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Option<Matrix<T>>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }
        let rows = self.0.len();
        let cols = self.0[0].len();

        let mut result = vec![vec![self.0[0][0]; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                result[i][j] = self.0[i][j] + rhs.0[i][j];
            }
        }
        Some(Matrix(result))
    }
}

impl<T: Scalar> Sub for Matrix<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Option<Matrix<T>>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }
        let rows = self.0.len();
        let cols = self.0[0].len();

        let mut result = vec![vec![self.0[0][0]; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                result[i][j] = self.0[i][j] - rhs.0[i][j];
            }
        }
        Some(Matrix(result))
    }
}
