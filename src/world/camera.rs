use crate::primitives::{point2d::Point2D, ray2d::Ray2D};

pub type Camera = Ray2D;

impl Camera {
    pub fn dummy() -> Camera {
        Ray2D::new(
            Point2D { x: -200.0, y: 0.0 },
            Point2D { x: -200.0, y: 10000.0 }
        )
    }

    pub fn ray_for_column(&self, column: u32) -> Ray2D {
        Ray2D {
            origin: self.origin,
            direction: Point2D { x: self.direction.x + f64::from(column), y: self.direction.y },
        }
    }
}