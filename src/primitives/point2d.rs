mod distance;
mod operators;
mod length;
mod normalise;
mod rotate;
mod tangent;
mod get_angle;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D{ x: x, y: y }
    }
}
