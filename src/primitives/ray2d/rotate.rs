use crate::primitives::{point2d::Point2D, ray2d::Ray2D};


impl Ray2D {
    pub fn rotate(&self, angle_radians: f64) -> Ray2D {
        let sin_angle = angle_radians.sin();
        let cos_angle = angle_radians.cos();
        let x = self.direction.x;
        let y = self.direction.y;

        let rotated_x = (x * cos_angle) - (y * sin_angle);
        let rotated_y = (x * sin_angle) + (y * cos_angle);

        Ray2D {
            origin: self.origin,
            direction: Point2D {x: rotated_x, y: rotated_y},
        }
    }
}