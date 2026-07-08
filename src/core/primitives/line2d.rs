mod get_angle;
mod intersection;
mod length;

use super::point2d::Point2D;

#[derive(Debug, Clone, Copy)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
}
