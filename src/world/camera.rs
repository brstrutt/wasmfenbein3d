use crate::primitives::{line2d::Line2D, point2d::Point2D, ray2d::Ray2D};

pub type Camera = Ray2D;

impl Camera {
    pub fn dummy() -> Camera {
        Ray2D::new(
            Point2D { x: -200.0, y: 0.0 },
            Point2D { x: -200.0, y: 10000.0 }
        )
    }

    pub fn ray_for_column(&self, column: u32) -> Line2D {
        Line2D {
            start: Point2D { x: self.origin.x + f64::from(column), y: self.origin.y },
            end: Point2D { x: self.direction.x + f64::from(column), y: self.direction.y },
        }
    }
}