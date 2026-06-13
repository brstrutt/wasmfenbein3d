use std::ops;

pub struct RGB {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl ops::Div<f64> for RGB {
    type Output = RGB;

    fn div(self, _rhs: f64) -> RGB {
        RGB {
            red: (self.red as f64 / _rhs) as u32,
            green: (self.green as f64 / _rhs) as u32,
            blue: (self.blue as f64 / _rhs) as u32,
        }
    }
}
