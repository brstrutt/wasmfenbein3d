use crate::primitives::{line2d::_Line2D, point2d::_Point2D};

impl _Line2D {
    // Implementation of a Line intersection point algorithm taken from https://web.archive.org/web/20060911055655/http://local.wasp.uwa.edu.au/~pbourke/geometry/lineline2d/
    pub fn _length(&self) -> f64 {
        let offset = _Point2D{x: (self.end.x - self.start.x).abs(), y: (self.end.y - self.start.y).abs()};
        ((offset.x * offset.x) + (offset.y * offset.y)).sqrt()
    }
}


#[cfg(test)]
mod line2d_length_tests {
    use super::*;

    #[test]
    fn test_vertical_lines() {
        let line = _Line2D{start: _Point2D{x: 0.0, y: 0.0}, end: _Point2D{x: 0.0, y: 3.4}};
        assert_eq!(line._length(), 3.4);
    }

    #[test]
    fn test_horizontal_lines() {
        let line = _Line2D{start: _Point2D{x: 0.0, y: 0.0}, end: _Point2D{x: 72.5, y: 0.0}};
        assert_eq!(line._length(), 72.5);
    }
    
    #[test]
    fn test_angled_lines() {
        let line = _Line2D{start: _Point2D{x: 0.0, y: 0.0}, end: _Point2D{x: 72.5, y: 10.4}};
        assert_eq!(line._length(), 73.24213268331282);
    }
}