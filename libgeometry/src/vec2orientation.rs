use std::ops::{Mul, Sub};

use crate::Vec2;

#[derive(PartialEq, Eq)]
pub enum Vec2Orientation {
    Collinear,
    Clockwise,
    Counterclockwise,
}

impl Vec2Orientation {
    // Get the orientation of the segment (p, q) and (q, r)
    pub fn from_points<T>(p: &Vec2<T>, q: &Vec2<T>, r: &Vec2<T>) -> Self
    where
        T: Sub + Copy,
        <T as Sub>::Output: Mul,
        <<T as Sub>::Output as Mul>::Output: Sub,
        <<<T as Sub>::Output as Mul>::Output as Sub>::Output: From<i8> + Eq + Ord,
    {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);

        if val == 0.into() {
            return Self::Collinear;
        };

        return if val > 0.into() {
            Self::Clockwise
        } else {
            Self::Counterclockwise
        };
    }
}
