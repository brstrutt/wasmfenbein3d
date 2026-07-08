use crate::core::primitives::point2d::Point2D;

impl Point2D {
    pub fn rotate(&self, angle_radians: f64) -> Point2D {
        let sin_angle = angle_radians.sin();
        let cos_angle = angle_radians.cos();
        let x = self.x;
        let y = self.y;

        let rotated_x = (x * cos_angle) - (y * sin_angle);
        let rotated_y = (x * sin_angle) + (y * cos_angle);

        Point2D {
            x: rotated_x,
            y: rotated_y,
        }
    }
}
