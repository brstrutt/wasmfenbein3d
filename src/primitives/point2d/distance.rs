use crate::primitives::point2d::_Point2D;


impl _Point2D {
    pub fn dist(start: &_Point2D, end: &_Point2D) -> f64 {
        let offset = _Point2D{x: (end.x - start.x).abs(), y: (end.y - start.y).abs()};
        ((offset.x * offset.x) + (offset.y * offset.y)).sqrt()
    }
}


#[cfg(test)]
mod point2d_dist_tests {
    use super::*;

    #[test]
    fn test_vertical_lines() {
        assert_eq!(_Point2D::dist(&_Point2D{x: 0.0, y: 0.0}, &_Point2D{x: 0.0, y: 3.4}), 3.4);
    }

    #[test]
    fn test_horizontal_lines() {
        assert_eq!(_Point2D::dist(&_Point2D{x: 0.0, y: 0.0}, &_Point2D{x: 72.5, y: 0.0}), 72.5);
    }
    
    #[test]
    fn test_angled_lines() {
        assert_eq!(_Point2D::dist(&_Point2D{x: 0.0, y: 0.0}, &_Point2D{x: 72.5, y: 10.4}), 73.24213268331282);
    }
}