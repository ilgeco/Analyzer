use std::ops;

pub trait Operation<'a>
where
    Self: Clone + Copy + PartialOrd,
    Self: ops::Sub<Output = Self> + ops::Add<Output = Self> + ops::Div<Output = Self>,
    Self: ops::Sub<&'a Self, Output = Self>
        + ops::Add<&'a Self, Output = Self>
        + ops::Div<&'a Self, Output = Self>,
    Self: std::iter::Sum<&'a Self>,
    Self: 'a,
{
    fn max(self, rgt: Self) -> Self;
    fn min(self, rgt: Self) -> Self;
    fn abs(self) -> Self;
    fn zero() -> Self;
    fn small_eps() -> Self;
    fn divu(self, rgt: usize) -> Self;
    fn mulu(self, rgt: usize) -> Self;
}

impl<'a> Operation<'a> for f64 {
    fn max(self, rgt: Self) -> Self {
        f64::max(self, rgt)
    }

    fn min(self, rgt: Self) -> Self {
        f64::min(self, rgt)
    }

    fn abs(self) -> Self {
        f64::abs(self)
    }

    fn zero() -> Self {
        0.0f64
    }

    fn small_eps() -> Self {
        0.000001f64
    }

    fn divu(self, rgt: usize) -> Self {
        self / rgt as f64
    }

    fn mulu(self, rgt: usize) -> Self {
        self * rgt as f64
    }
}
