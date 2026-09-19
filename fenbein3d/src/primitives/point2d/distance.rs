use super::Point2D;

impl Point2D {
    pub fn dist(start: &Point2D, end: &Point2D) -> f64 {
        (*end - *start).abs().length()
    }
}
