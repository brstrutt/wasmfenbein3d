use crate::primitives::point2d::Point2D;


impl Point2D {
    pub fn dist(start: &Point2D, end: &Point2D) -> f64 {
        let offset = (*end - *start).abs();
        ((offset.x * offset.x) + (offset.y * offset.y)).sqrt()
    }
}


#[cfg(test)]
mod point2d_dist_tests {
    use super::*;

    #[test]
    fn test_vertical_lines() {
        assert_eq!(Point2D::dist(&Point2D{x: 0.0, y: 0.0}, &Point2D{x: 0.0, y: 3.4}), 3.4);
    }

    #[test]
    fn test_horizontal_lines() {
        assert_eq!(Point2D::dist(&Point2D{x: 0.0, y: 0.0}, &Point2D{x: 72.5, y: 0.0}), 72.5);
    }
    
    #[test]
    fn test_angled_lines() {
        assert_eq!(Point2D::dist(&Point2D{x: 0.0, y: 0.0}, &Point2D{x: 72.5, y: 10.4}), 73.24213268331282);
    }
}