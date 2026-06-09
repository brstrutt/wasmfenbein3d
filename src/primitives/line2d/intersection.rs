use crate::primitives::{line2d::_Line2D, point2d::_Point2D};

impl _Line2D {
    // Implementation of a Line intersection point algorithm taken from https://web.archive.org/web/20060911055655/http://local.wasp.uwa.edu.au/~pbourke/geometry/lineline2d/
    pub fn _intersection(&self, other: _Line2D) -> Option<_Point2D> {
        let y1 = self.start.y;
        let y2 = self.end.y;
        let y3 = other.start.y;
        let y4 = other.end.y;
        let x1 = self.start.x;
        let x2 = self.end.x;
        let x3 = other.start.x;
        let x4 = other.end.x;

        let denominator = ((y4 - y3) * (x2 - x1)) - ((x4 - x3) * (y2 - y1));

        if denominator == 0.0 {
            return None;
        }

        let ua_numerator = ((x4 - x3) * (y1 - y3)) - ((y4 - y3) * (x1 - x3));
        let ua = ua_numerator / denominator;

        if ua > 1.0 || ua < 0.0 {
            return None;
        }

        let ub_numerator = ((x2 - x1) * (y1 - y3)) - ((y2 - y1) * (x1 - x3));
        let ub = ub_numerator / denominator;

        if ub > 1.0 || ub < 0.0 {
            return None;
        }

        let x = x1 + (ua * (x2 - x1));
        let y = y1 + (ua * (y2 - y1));

        Some(_Point2D { x, y })
    }
}

#[cfg(test)]
mod line2d_intersection_tests {
    use super::*;

    #[test]
    fn test_parallel_lines() {
        let line1 = _Line2D {
            start: _Point2D { x: 0.0, y: 0.0 },
            end: _Point2D { x: 0.0, y: 10.0 },
        };
        let line2 = _Line2D {
            start: _Point2D { x: 10.0, y: 0.0 },
            end: _Point2D { x: 10.0, y: 10.0 },
        };

        assert_eq!(line1._intersection(line2).is_none(), true);
    }

    #[test]
    fn test_perpendicular_lines() {
        let line1 = _Line2D {
            start: _Point2D { x: 0.0, y: -10.0 },
            end: _Point2D { x: 0.0, y: 10.0 },
        };
        let line2 = _Line2D {
            start: _Point2D { x: -10.0, y: 0.0 },
            end: _Point2D { x: 10.0, y: 0.0 },
        };

        assert_eq!(
            line1._intersection(line2).unwrap(),
            _Point2D { x: 0.0, y: 0.0 }
        );
    }

    #[test]
    fn test_angled_lines() {
        let intersection = _Point2D { x: 2.0, y: 1.0 };

        let line1 = _Line2D {
            start: _Point2D {
                x: intersection.x + 4.0,
                y: intersection.y - 10.0,
            },
            end: _Point2D {
                x: intersection.x - 4.0,
                y: intersection.y + 10.0,
            },
        };
        let line2 = _Line2D {
            start: _Point2D { x: -232.0, y: 1.0 },
            end: _Point2D { x: 21.0, y: 1.0 },
        };

        assert_eq!(
            line1._intersection(line2).unwrap(),
            _Point2D { x: 2.0, y: 1.0 }
        );
    }

    #[test]
    fn test_lines_with_intersection_outside_bounds() {
        let line1 = _Line2D {
            start: _Point2D { x: 0.0, y: -10.0 },
            end: _Point2D { x: 0.0, y: 10.0 },
        };
        let line2 = _Line2D {
            start: _Point2D { x: 10.0, y: 0.0 },
            end: _Point2D { x: 30.0, y: 0.0 },
        };

        assert_eq!(line1._intersection(line2).is_none(), true);
    }
}
