use super::super::{line2d::Line2D, point2d::Point2D};
use super::Ray2D;

impl Ray2D {
    pub fn intersection(&self, other: &Line2D) -> Option<Point2D> {
        Line2D {
            start: self.origin,
            end: (self.direction
                * Point2D {
                    x: 1_000_000.0,
                    y: 1_000_000.0,
                })
                + self.origin,
        }
        .intersection(other)
    }
}
