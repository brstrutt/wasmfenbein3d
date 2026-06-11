use crate::primitives::{line2d::Line2D, point2d::Point2D};

#[derive(Clone, Copy)]
pub struct Camera {
    pub position: Point2D,
    direction: Point2D,
}

impl Camera {
    pub fn dummy() -> Camera {
        Camera {
            position: Point2D { x: -200.0, y: 0.0 },
            direction: Point2D { x: -200.0, y: 10000.0 },
        }
    }

    pub fn ray_for_column(&self, column: u32) -> Line2D {
        Line2D {
            start: Point2D { x: self.position.x + f64::from(column), y: self.position.y },
            end: Point2D { x: self.direction.x + f64::from(column), y: self.direction.y },
        }
    }
}