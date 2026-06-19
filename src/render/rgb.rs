use std::ops;

pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

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
