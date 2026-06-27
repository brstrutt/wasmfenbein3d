use crate::primitives::ray2d::Ray2D;


impl Ray2D {
    pub fn rotate(&self, angle_radians: f64) -> Ray2D {
        Ray2D {
            origin: self.origin,
            direction: self.direction.rotate(angle_radians),
        }
    }
}