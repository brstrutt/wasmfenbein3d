use crate::primitives::point2d::Point2D;

impl Point2D {
    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
}

#[cfg(test)]
mod point2d_length_tests {
use super::*;

    #[test]
    fn test_vertical_line() {
        assert_eq!(Point2D{x: 0.0, y: 3.4}.length(), 3.4);
    }

    #[test]
    fn test_horizontal_line() {
        assert_eq!(Point2D{x: 72.5, y: 0.0}.length(), 72.5);
    }
    
    #[test]
    fn test_angled_line() {
        assert_eq!(Point2D{x: 72.5, y: 10.4}.length(), 73.24213268331282);
    }
}