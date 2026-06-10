mod intersection;
mod length;

use crate::primitives::point2d::Point2D;

#[derive(Debug)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
}
