mod intersection;
mod length;

use crate::primitives::point2d::Point2D;

#[derive(Debug, Clone, Copy)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
}

impl Line2D {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Line2D{start: Point2D{ x: x1, y: y1 }, end: Point2D { x: x2, y: y2 }}
    }
}
