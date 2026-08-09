use std::ops;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RGB {
    pub fn from_u8(rgb_bytes: &[u8; 3]) -> Self {
        RGB {
            red: rgb_bytes[0],
            green: rgb_bytes[1],
            blue: rgb_bytes[2],
        }
    }
}

pub const WHITE: RGB = RGB {
    red: 255,
    green: 255,
    blue: 255,
};

pub const ERROR: RGB = RGB {
    red: 255,
    green: 0,
    blue: 0,
};

impl ops::Div<f64> for RGB {
    type Output = RGB;

    fn div(self, _rhs: f64) -> RGB {
        RGB {
            red: (self.red as f64 / _rhs) as u8,
            green: (self.green as f64 / _rhs) as u8,
            blue: (self.blue as f64 / _rhs) as u8,
        }
    }
}

impl ops::Div<f64> for &RGB {
    type Output = RGB;

    fn div(self, _rhs: f64) -> RGB {
        RGB {
            red: (self.red as f64 / _rhs) as u8,
            green: (self.green as f64 / _rhs) as u8,
            blue: (self.blue as f64 / _rhs) as u8,
        }
    }
}

impl ops::Sub<u8> for &RGB {
    type Output = RGB;

    fn sub(self, _rhs: u8) -> RGB {
        RGB {
            red: self.red.saturating_sub(_rhs),
            green: self.green.saturating_sub(_rhs),
            blue: self.blue.saturating_sub(_rhs),
        }
    }
}
