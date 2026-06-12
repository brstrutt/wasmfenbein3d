use crate::primitives::point2d::Point2D;

#[derive(Debug, Clone, Copy)]
pub struct Ray2D {
    pub origin: Point2D,
    pub direction: Point2D,
}

impl Ray2D {
    pub fn new(origin: Point2D, direction: Point2D) -> Ray2D {
        Ray2D {
            origin,
            direction: direction.normalise(),
        }
    }
}

