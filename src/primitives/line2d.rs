mod intersection;
mod length;

use crate::primitives::point2d::_Point2D;

#[derive(Debug)]
struct _Line2D {
    start: _Point2D,
    end: _Point2D,
}
