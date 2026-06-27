use crate::primitives::ray2d::Ray2D;

impl Ray2D {
    pub fn tangent(&self) -> Ray2D {
        Ray2D {
            origin: self.origin,
            direction: self.direction.tangent(),
        }
    }
}
