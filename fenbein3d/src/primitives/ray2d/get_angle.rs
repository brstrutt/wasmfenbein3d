use super::Ray2D;

impl Ray2D {
    pub fn get_angle(&self) -> f64 {
        self.direction.get_angle()
    }
}
