use std::ops::{Div, Mul, Sub};

use crate::{Vec2, segment::Segment};

/// An axis-aligned rectangular region
pub struct Rect<T> {
    topright: Vec2<T>,
    bottomleft: Vec2<T>,
}

impl<T> Rect<T> {
    pub fn new(a: &Vec2<T>, b: &Vec2<T>) -> Self
    where
        T: Copy + Ord,
    {
        Self {
            topright: a.max_coords(b),
            bottomleft: a.min_coords(b),
        }
    }
    pub fn new_from_segment(segment: &Segment<T>) -> Self
    where
        T: Copy + Ord,
    {
        Self::new(segment.start(), segment.end())
    }

    // Returns true if the point is inside the region
    pub fn point_in_region(&self, point: &Vec2<T>) -> bool
    where
        T: Ord,
    {
        self.bottomleft.x <= point.x
            && point.x <= self.topright.x
            && self.bottomleft.y <= point.y
            && point.y <= self.topright.y
    }
    // Returns true if the point is inside the rect (excluding the border)
    pub fn point_in_region_exclusive(&self, point: &Vec2<T>) -> bool
    where
        T: Ord,
    {
        self.bottomleft.x < point.x
            && point.x < self.topright.x
            && self.bottomleft.y < point.y
            && point.y < self.topright.y
    }

    pub fn segment_top(&self) -> Segment<T>
    where
        T: Copy,
    {
        Segment::new(
            Vec2::new(self.bottomleft.x, self.topright.y),
            Vec2::new(self.topright.x, self.topright.y),
        )
    }

    pub fn segment_bottom(&self) -> Segment<T>
    where
        T: Copy,
    {
        Segment::new(
            Vec2::new(self.bottomleft.x, self.bottomleft.y),
            Vec2::new(self.topright.x, self.bottomleft.y),
        )
    }

    pub fn segment_right(&self) -> Segment<T>
    where
        T: Copy,
    {
        Segment::new(
            Vec2::new(self.topright.x, self.bottomleft.y),
            Vec2::new(self.topright.x, self.topright.y),
        )
    }

    pub fn segment_left(&self) -> Segment<T>
    where
        T: Copy,
    {
        Segment::new(
            Vec2::new(self.bottomleft.x, self.bottomleft.y),
            Vec2::new(self.bottomleft.x, self.topright.y),
        )
    }

    /// Returns the segment that is intersecting this region, if the passed segment intersects.
    /// Returns None if the passed segment does not intersect.
    pub fn intersects_segment(&self, segment: &Segment<T>) -> Option<Segment<T>>
    where
        T: Ord + Copy + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<i8> + Eq,
    {
        let start_in_region = self.point_in_region_exclusive(segment.start());
        let end_in_region = self.point_in_region_exclusive(segment.end());
        // special case: segment is wholly contained
        if start_in_region && end_in_region {
            return Some(*segment);
        }

        let intersection_top = self.segment_top().intersects_segment(segment);
        let intersection_bottom = self.segment_bottom().intersects_segment(segment);
        let intersection_right = self.segment_right().intersects_segment(segment);
        let intersection_left = self.segment_left().intersects_segment(segment);

        if !start_in_region
            && !end_in_region
            && !intersection_bottom.is_some()
            && !intersection_left.is_some()
            && !intersection_right.is_some()
            && !intersection_top.is_some()
        {
            // no intersection at all
            return None;
        }
        let mut intersections = vec![];
        if start_in_region {
            intersections.push(segment.start());
        }
        if end_in_region {
            intersections.push(segment.end());
        }
        if let Some(ref intersection) = intersection_bottom {
            intersections.push(intersection);
        }
        if let Some(ref intersection) = intersection_top {
            intersections.push(intersection);
        }
        if let Some(ref intersection) = intersection_left {
            intersections.push(intersection);
        }
        if let Some(ref intersection) = intersection_right {
            intersections.push(intersection);
        }
        assert_eq!(
            intersections.len(),
            2,
            "Segment-rect intersection implementation is wrong"
        );
        return Some(Segment::new(
            *intersections.pop().unwrap(),
            *intersections.pop().unwrap(),
        ));
    }
}
