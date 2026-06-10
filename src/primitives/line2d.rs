mod intersection;
mod length;

use crate::primitives::point2d::_Point2D;

#[derive(Debug)]
pub struct _Line2D {
    pub start: _Point2D,
    pub end: _Point2D,
}
