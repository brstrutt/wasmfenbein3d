use crate::core::primitives::line2d::Line2D;

impl Line2D {
    pub fn get_angle(&self) -> f64 {
        (self.end - self.start).get_angle()
    }
}
