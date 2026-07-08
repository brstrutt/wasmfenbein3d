use std::{fmt, ops};

use crate::core::primitives::point2d::Point2D;

impl ops::Add<Point2D> for Point2D {
    type Output = Point2D;

    fn add(self, _rhs: Point2D) -> Point2D {
        Point2D {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Add<&Point2D> for &Point2D {
    type Output = Point2D;

    fn add(self, _rhs: &Point2D) -> Point2D {
        Point2D {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::AddAssign for Point2D {
    fn add_assign(&mut self, rhs: Point2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<Point2D> for Point2D {
    type Output = Point2D;

    fn sub(self, _rhs: Point2D) -> Point2D {
        Point2D {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl ops::SubAssign for Point2D {
    fn sub_assign(&mut self, rhs: Point2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<Point2D> for Point2D {
    type Output = Point2D;

    fn mul(self, _rhs: Point2D) -> Point2D {
        Point2D {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
        }
    }
}

impl ops::Mul<f64> for Point2D {
    type Output = Point2D;

    fn mul(self, _rhs: f64) -> Point2D {
        Point2D {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

impl ops::Div<Point2D> for Point2D {
    type Output = Point2D;

    fn div(self, _rhs: Point2D) -> Point2D {
        Point2D {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
        }
    }
}

impl Point2D {
    pub fn abs(&self) -> Point2D {
        Point2D {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
