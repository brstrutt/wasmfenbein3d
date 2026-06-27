use crate::primitives::point2d::Point2D;

impl Point2D {
    pub fn get_angle(&self) -> f64 {
        let gradient = self.x/self.y;
        gradient.atan() * -1.0
    }
}


#[cfg(test)]
mod test {
use super::*;

    #[test]
    fn get_angle_is_the_inverse_of_rotating_the_y_axis() {
        let point = Point2D{x: 0.0, y: 100.0};
        let original_angle = std::f32::consts::PI as f64 / 3.4;

        assert_eq!(point.rotate(original_angle).get_angle(), original_angle);
    }
}
