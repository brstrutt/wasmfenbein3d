use crate::primitives::{line2d::Line2D, point2d::Point2D};

impl Line2D {
    pub fn _length(&self) -> f64 {
        Point2D::dist(&self.start, &self.end)
    }
}