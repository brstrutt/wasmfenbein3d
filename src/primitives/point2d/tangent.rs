use crate::primitives::point2d::Point2D;

impl Point2D {
    pub fn tangent(&self) -> Point2D {
        self.rotate(std::f32::consts::PI as f64 / 2.0)
    }
}
