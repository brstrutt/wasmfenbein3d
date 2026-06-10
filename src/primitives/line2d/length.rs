use crate::primitives::{line2d::_Line2D, point2d::_Point2D};

impl _Line2D {
    // Implementation of a Line intersection point algorithm taken from https://web.archive.org/web/20060911055655/http://local.wasp.uwa.edu.au/~pbourke/geometry/lineline2d/
    pub fn _length(&self) -> f64 {
        _Point2D::dist(&self.start, &self.end)
    }
}