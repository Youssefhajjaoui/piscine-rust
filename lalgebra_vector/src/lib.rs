use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

pub trait Scalar: Add<Output = Self> + Mul<Output = Self> + Clone + Debug + PartialEq + Eq {
    fn zero() -> Self;
}
impl Scalar for i32 {
    fn zero() -> Self {
        0
    }
}
impl Scalar for i64 {
    fn zero() -> Self {
        0
    }
}
// impl Scalar for f32 {
//     fn zero() -> Self {
//         0.
//     }
// }
// impl Scalar for f64 {
//     fn zero() -> Self {
//         0.
//     }
// }
impl Scalar for i8 {
    fn zero() -> Self {
        0
    }
}
impl Scalar for i16 {
    fn zero() -> Self {
        0
    }
}
impl Scalar for u8 {
    fn zero() -> Self {
        0
    }
}
impl Scalar for u16 {
    fn zero() -> Self {
        0
    }
}
impl Scalar for u32 {
    fn zero() -> Self {
        0
    }
}
impl Scalar for u64 {
    fn zero() -> Self {
        0
    }
}
#[derive(Debug)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // if self.0.len() != rhs.0.len() {
        //     return None;
        // }
        let data = self
            .0
            .into_iter()
            .zip(rhs.0.into_iter())
            .map(|(a, b)| a + b)
            .collect();
        Vector(data)
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut sum = T::zero();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            sum = sum + a.clone() * b.clone();
        }
        Some(sum)
    }
}
