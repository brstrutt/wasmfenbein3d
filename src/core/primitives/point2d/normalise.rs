use crate::core::primitives::point2d::Point2D;

impl Point2D {
    pub fn normalise(&self) -> Point2D {
        let length = self.length();
        let normalised_x = self.x / length;
        let normalised_y = self.y / length;

        Point2D {
            x: normalised_x,
            y: normalised_y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result_has_length_1() {
        let point = Point2D { x: 23.6, y: 28.54 };
        assert_eq!(point.normalise().length(), 1.0);
    }
}
