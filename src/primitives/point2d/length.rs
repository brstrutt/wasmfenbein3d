use crate::primitives::point2d::Point2D;

impl Point2D {
    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vertical_line() {
        assert_eq!(Point2D { x: 0.0, y: 3.4 }.length(), 3.4);
    }

    #[test]
    fn horizontal_line() {
        assert_eq!(Point2D { x: 72.5, y: 0.0 }.length(), 72.5);
    }

    #[test]
    fn angled_line() {
        assert_eq!(Point2D { x: 72.5, y: 10.4 }.length(), 73.24213268331282);
    }
}
