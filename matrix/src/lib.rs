#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

pub trait Scalar {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for i32 {
    type Item = i32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for f32 {
    type Item = f32;
    fn zero() -> Self::Item {
        0.
    }
    fn one() -> Self::Item {
        1.
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
impl Scalar for u64 {
    type Item = u64;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
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

impl<T: Scalar<Item = T> + Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Self(vec![vec![]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let colomn: Vec<T> = vec![T::zero(); col];
        let rows: Vec<_> = vec![colomn; row];
        Self(rows)
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut res: Vec<_> = vec![];
        for i in 0..n {
            let mut line: Vec<T> = vec![];
            for j in 0..n {
                if i == j {
                    line.push(T::one());
                } else {
                    line.push(T::zero());
                }
            }
            res.push(line);
        }
        return Matrix(res);
    }
}
