use std::ops::{Add, Sub};
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
    pub i: T,
    pub j: T,
    pub k: T,
}

impl<T> Add for ThreeDVector<T>
where
    T: Add<Output = T>,
{
    type Output = ThreeDVector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

impl<T> Sub for ThreeDVector<T>
where
    T: Sub<Output = T>,
{
    type Output = ThreeDVector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        {
            Self {
                i: self.i - rhs.i,
                j: self.j - rhs.j,
                k: self.k - rhs.k,
            }
        }
    }
}
