use crate::primitives::point2d::_Point2D;

pub struct Camera {
    position: _Point2D,
    direction: _Point2D,
}

impl Camera {
    pub fn dummy() -> Camera {
        Camera {
            position: _Point2D { x: 0.0, y: 0.0 },
            direction: _Point2D { x: 100.0, y: 100.0 },
        }
    }
}