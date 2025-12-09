use std::ops::{Add, Mul};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point2<T> {
    x: T,
    y: T,
}

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Point2<T> {
        Self { x, y }
    }
    pub fn zero() -> Point2<T>
    where
        T: From<i32>,
    {
        Self {
            x: 0.into(),
            y: 0.into(),
        }
    }
}

impl<T> Add for Point2<T>
where
    T: Add,
{
    type Output = Point2<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T, Tscalar> Mul<Tscalar> for Point2<T>
where
    T: Mul<Tscalar>,
    Tscalar: Copy,
{
    type Output = Point2<<T as Mul<Tscalar>>::Output>;

    fn mul(self, rhs: Tscalar) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Point2<T> {
    pub fn dot<TOther, TResult, TMiddle>(
        self,
        rhs: Point2<TOther>,
    ) -> <<T as Mul<TOther>>::Output as Add>::Output
    where
        T: Mul<TOther>,
        <T as Mul<TOther>>::Output: Add,
    {
        self.x * rhs.x + self.y * rhs.y
    }
}
