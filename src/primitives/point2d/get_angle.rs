use crate::primitives::point2d::Point2D;

impl Point2D {
    pub fn get_angle(&self) -> f64 {
        let gradient = self.x/self.y;
        gradient.atan() * -1.0
    }
}


#[cfg(test)]
mod test {
use crate::utils::assert_floats_equal::assert_floats_equal;

use super::*;

    #[test]
    fn get_angle_is_the_inverse_of_rotating_the_y_axis() {
        test_rotation_angle_is_returned_by_get_angle(std::f32::consts::PI as f64 / 3.4);
        test_rotation_angle_is_returned_by_get_angle(std::f32::consts::PI as f64 / 7.3);
        test_rotation_angle_is_returned_by_get_angle(std::f32::consts::PI as f64 / -1.3);
        test_rotation_angle_is_returned_by_get_angle(std::f32::consts::PI as f64 / 2.0);
    }
    fn test_rotation_angle_is_returned_by_get_angle(original_angle: f64) {
        let point = Point2D{x: 0.0, y: 100.0};

        assert_floats_equal(point.rotate(original_angle).get_angle(), original_angle);
    }
}
