mod distance;
mod operators;
mod length;
mod normalise;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}
