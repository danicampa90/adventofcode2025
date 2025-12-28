use std::ops::{Div, Mul, Sub};

use crate::{Rect, Vec2, vec2orientation::Vec2Orientation};

#[derive(Clone, Copy)]
pub struct Segment<T> {
    start: Vec2<T>,
    end: Vec2<T>,
}

impl<T> Segment<T> {
    pub fn start(&self) -> &Vec2<T> {
        &self.start
    }
    pub fn end(&self) -> &Vec2<T> {
        &self.end
    }
    pub fn new(start: Vec2<T>, end: Vec2<T>) -> Self {
        Self { start, end }
    }
    // returns the point of intersection between this segment and another segment, if the two segments intersect.
    // Handles also the cases where segments are vertical.
    pub fn intersects_segment(&self, other: &Self) -> Option<Vec2<T>>
    where
        T: Ord + Copy + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<i8> + Eq,
    {
        let p1 = self.start;
        let p2 = self.end;
        let p3 = other.start;
        let p4 = other.end;

        let o1 = Vec2Orientation::from_points(&p1, &p2, &p3);
        let o2 = Vec2Orientation::from_points(&p1, &p2, &p4);
        let o3 = Vec2Orientation::from_points(&p3, &p4, &p1);
        let o4 = Vec2Orientation::from_points(&p3, &p4, &p2);

        // general case - segments intersect at a single point
        if o1 != o2 && o3 != o4 {
            // Calculate intersection point using line intersection formula
            // Using formula:
            // x = (x1*y2 - y1*x2)*(x3-x4) - (x1-x2)*(x3*y4 - y3*x4) / denominator
            // y = (x1*y2 - y1*x2)*(y3-y4) - (y1-y2)*(x3*y4 - y3*x4) / denominator
            // where denominator = (x1-x2)*(y3-y4) - (y1-y2)*(x3-x4)

            let x1y2 = p1.x * p2.y;
            let y1x2 = p1.y * p2.x;
            let x3y4 = p3.x * p4.y;
            let y3x4 = p3.y * p4.x;

            let denominator = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);

            // If denominator is 0, lines are parallel (shouldn't happen in general case)
            if denominator == T::from(0) {
                return None;
            }

            let x_numerator = (x1y2 - y1x2) * (p3.x - p4.x) - (p1.x - p2.x) * (x3y4 - y3x4);
            let y_numerator = (x1y2 - y1x2) * (p3.y - p4.y) - (p1.y - p2.y) * (x3y4 - y3x4);

            let x = x_numerator / denominator;
            let y = y_numerator / denominator;

            return Some(Vec2::new(x, y));
        }

        let self_rect = Rect::new(&p1, &p2);

        // special cases - collinear segments
        // p1, q1 and p2 are collinear and p2 lies on segment p1q1
        if o1 == Vec2Orientation::Collinear && self_rect.point_in_region(&p3) {
            return Some(p3);
        }

        // p1, q1 and q2 are collinear and q2 lies on segment p1q1
        if o2 == Vec2Orientation::Collinear && self_rect.point_in_region(&p4) {
            return Some(p4);
        }

        let other_rect = Rect::new(&p3, &p4);

        // p2, q2 and p1 are collinear and p1 lies on segment p2q2
        if o3 == Vec2Orientation::Collinear && other_rect.point_in_region(&p1) {
            return Some(p1);
        }

        // p2, q2 and q1 are collinear and q1 lies on segment p2q2
        if o4 == Vec2Orientation::Collinear && other_rect.point_in_region(&p2) {
            return Some(p2);
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_intersection_general_case() {
        // Test case 1: Simple intersection
        let seg1 = Segment::new(Vec2::new(0, 0), Vec2::new(2, 2));
        let seg2 = Segment::new(Vec2::new(0, 2), Vec2::new(2, 0));
        let intersection = seg1.intersects_segment(&seg2);
        assert_eq!(intersection, Some(Vec2::new(1, 1)));

        // Test case 2: No intersection
        let seg1 = Segment::new(Vec2::new(0, 0), Vec2::new(1, 1));
        let seg2 = Segment::new(Vec2::new(2, 2), Vec2::new(3, 3));
        let intersection = seg1.intersects_segment(&seg2);
        assert_eq!(intersection, None);

        // Test case 3: Parallel segments (no intersection)
        let seg1 = Segment::new(Vec2::new(0, 0), Vec2::new(2, 0));
        let seg2 = Segment::new(Vec2::new(0, 1), Vec2::new(2, 1));
        let intersection = seg1.intersects_segment(&seg2);
        assert_eq!(intersection, None);
    }

    #[test]
    fn test_segment_intersection_collinear_cases() {
        // Test case 1: Collinear with endpoint on segment
        let seg1 = Segment::new(Vec2::new(0, 0), Vec2::new(4, 0));
        let seg2 = Segment::new(Vec2::new(2, 0), Vec2::new(6, 0));
        let intersection = seg1.intersects_segment(&seg2);
        assert_eq!(intersection, Some(Vec2::new(2, 0)));

        // Test case 2: Collinear segments that don't overlap
        let seg1 = Segment::new(Vec2::new(0, 0), Vec2::new(2, 0));
        let seg2 = Segment::new(Vec2::new(3, 0), Vec2::new(5, 0));
        let intersection = seg1.intersects_segment(&seg2);
        assert_eq!(intersection, None);

        // Test case 3: One segment completely inside another
        let seg1 = Segment::new(Vec2::new(0, 0), Vec2::new(6, 0));
        let seg2 = Segment::new(Vec2::new(2, 0), Vec2::new(4, 0));
        let intersection = seg1.intersects_segment(&seg2);
        assert_eq!(intersection, Some(Vec2::new(2, 0)));
    }

    #[test]
    fn test_segment_intersection_vertical_horizontal() {
        // Test case 1: Vertical and horizontal segments intersecting
        let seg1 = Segment::new(Vec2::new(2, 0), Vec2::new(2, 4));
        let seg2 = Segment::new(Vec2::new(0, 2), Vec2::new(4, 2));
        let intersection = seg1.intersects_segment(&seg2);
        assert_eq!(intersection, Some(Vec2::new(2, 2)));

        // Test case 2: Vertical segments (parallel, no intersection)
        let seg1 = Segment::new(Vec2::new(1, 0), Vec2::new(1, 4));
        let seg2 = Segment::new(Vec2::new(2, 0), Vec2::new(2, 4));
        let intersection = seg1.intersects_segment(&seg2);
        assert_eq!(intersection, None);
    }

    #[test]
    fn test_segment_intersection_touching_at_endpoints() {
        // Test case 1: Segments touching at endpoints
        let seg1 = Segment::new(Vec2::new(0, 0), Vec2::new(2, 0));
        let seg2 = Segment::new(Vec2::new(2, 0), Vec2::new(2, 2));
        let intersection = seg1.intersects_segment(&seg2);
        assert_eq!(intersection, Some(Vec2::new(2, 0)));

        // Test case 2: Segments sharing an endpoint but not intersecting
        let seg1 = Segment::new(Vec2::new(0, 0), Vec2::new(2, 0));
        let seg2 = Segment::new(Vec2::new(2, 0), Vec2::new(4, 0));
        let intersection = seg1.intersects_segment(&seg2);
        assert_eq!(intersection, Some(Vec2::new(2, 0)));
    }
}
