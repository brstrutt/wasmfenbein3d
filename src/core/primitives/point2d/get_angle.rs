use std::f64::consts::PI;

use super::Point2D;

impl Point2D {
    pub fn get_angle(&self) -> f64 {
        let gradient = self.x / self.y;
        let angle_to_y_axis = -gradient.atan();

        if self.y >= 0.0 {
            angle_to_y_axis
        } else {
            (PI - angle_to_y_axis.abs()) * -angle_to_y_axis.signum()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::utils::assert_floats_equal::tests::assert_floats_equal;

    use super::*;

    #[test]
    fn get_angle_is_the_inverse_of_rotating_the_y_axis_less_than_180_degrees() {
        test_get_angle_is_the_inverse_of_rotating_the_y_axis_by_amount(2.0 * PI / -3.4);
        test_get_angle_is_the_inverse_of_rotating_the_y_axis_by_amount(2.0 * PI / 2.2);
        test_get_angle_is_the_inverse_of_rotating_the_y_axis_by_amount(2.0 * PI / 4.0);
        test_get_angle_is_the_inverse_of_rotating_the_y_axis_by_amount(2.0 * PI / 7.9);
    }
    fn test_get_angle_is_the_inverse_of_rotating_the_y_axis_by_amount(original_angle: f64) {
        let point = Point2D { x: 0.0, y: 100.0 };

        assert_floats_equal(point.rotate(original_angle).get_angle(), original_angle);
    }
}
