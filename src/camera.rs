use crate::primitives::{line2d::_Line2D, point2d::_Point2D};

pub struct Camera {
    position: _Point2D,
    direction: _Point2D,
}

impl Camera {
    pub fn dummy() -> Camera {
        Camera {
            position: _Point2D { x: 0.0, y: 0.0 },
            direction: _Point2D { x: 0.0, y: 100.0 },
        }
    }

    pub fn ray_for_column(&self, column: u32) -> _Line2D {
        _Line2D {
            start: _Point2D { x: self.position.x + f64::from(column), y: self.position.y },
            end: _Point2D { x: self.direction.x + f64::from(column), y: self.direction.y },
        }
    }
}