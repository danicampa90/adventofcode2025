use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

pub type Vec2f = Vec2<f64>;
pub type Vec2i = Vec2<i64>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Debug for Vec2<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Self { x, y }
    }
    pub fn zero() -> Vec2<T>
    where
        T: From<i32>,
    {
        Self {
            x: 0.into(),
            y: 0.into(),
        }
    }

    pub fn dot<TOther, TResult>(
        self,
        rhs: Vec2<TOther>,
    ) -> <<T as Mul<TOther>>::Output as Add>::Output
    where
        T: Mul<TOther>,
        <T as Mul<TOther>>::Output: Add,
    {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn squared_magnitude(&self) -> <<T as Mul<T>>::Output as Add>::Output
    where
        T: Mul<T> + Copy,
        <T as Mul<T>>::Output: Add,
    {
        self.x * self.x + self.y * self.y
    }

    pub fn area(&self) -> T
    where
        T: Mul<T, Output = T> + Copy + From<i8> + Ord + Sub<Output = T>,
    {
        let area = self.x * self.y;
        let zero = T::from(0);
        if area < zero {
            return zero - area;
        } else {
            return area;
        }
    }
    pub fn area_inclusive(&self) -> T
    where
        T: Mul<T, Output = T> + Copy + From<i8> + Ord + Sub<Output = T> + Add<Output = T>,
    {
        let zero = T::from(0);
        let one = T::from(1);
        let x = if self.x < zero {
            zero - self.x + one
        } else {
            self.x + one
        };
        let y = if self.y < zero {
            zero - self.y + one
        } else {
            self.y + one
        };
        x * y
    }

    pub fn max_coords(&self, other: &Self) -> Self
    where
        T: Ord + Copy,
    {
        let x = if other.x > self.x { other.x } else { self.x };
        let y = if other.y > self.y { other.y } else { self.y };
        Self { x, y }
    }
    pub fn min_coords(&self, other: &Self) -> Self
    where
        T: Ord + Copy,
    {
        let x = if other.x < self.x { other.x } else { self.x };
        let y = if other.y < self.y { other.y } else { self.y };
        Self { x, y }
    }
}

impl<T> Add for Vec2<T>
where
    T: Add,
{
    type Output = Vec2<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vec2<T>
where
    T: Sub,
{
    type Output = Vec2<<T as Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T, Tscalar> Mul<Tscalar> for Vec2<T>
where
    T: Mul<Tscalar>,
    Tscalar: Copy,
{
    type Output = Vec2<<T as Mul<Tscalar>>::Output>;

    fn mul(self, rhs: Tscalar) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
