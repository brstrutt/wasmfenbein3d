use crate::primitives::{line2d::_Line2D, point2d::_Point2D};

impl _Line2D {
    pub fn _length(&self) -> f64 {
        _Point2D::dist(&self.start, &self.end)
    }
}