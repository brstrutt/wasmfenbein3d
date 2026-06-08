use crate::primitives::{line2d::Line2D, point2d::Point2D};

impl Line2D {
    // Implementation of a Line intersection point algorithm taken from https://web.archive.org/web/20060911055655/http://local.wasp.uwa.edu.au/~pbourke/geometry/lineline2d/
    pub fn length(&self) -> f64 {
        let offset = Point2D{x: (self.end.x - self.start.x).abs(), y: (self.end.y - self.start.y).abs()};
        ((offset.x * offset.x) + (offset.y * offset.y)).sqrt()
    }
}


#[cfg(test)]
mod line2d_length_tests {
    use super::*;

    #[test]
    fn test_vertical_lines() {
        let line = Line2D{start: Point2D{x: 0.0, y: 0.0}, end: Point2D{x: 0.0, y: 3.4}};
        assert_eq!(line.length(), 3.4);
    }

    #[test]
    fn test_horizontal_lines() {
        let line = Line2D{start: Point2D{x: 0.0, y: 0.0}, end: Point2D{x: 72.5, y: 0.0}};
        assert_eq!(line.length(), 72.5);
    }
    
    #[test]
    fn test_angled_lines() {
        let line = Line2D{start: Point2D{x: 0.0, y: 0.0}, end: Point2D{x: 72.5, y: 10.4}};
        assert_eq!(line.length(), 73.24213268331282);
    }
}