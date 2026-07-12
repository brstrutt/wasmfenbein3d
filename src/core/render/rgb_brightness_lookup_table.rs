use super::rgb::{RGB, WHITE};

#[derive(Clone)]
pub struct RgbBrightnessLookupTable {
    pub values: [RGB; 100],
}

impl RgbBrightnessLookupTable {
    pub fn generate(max_value: &RGB) -> Self {
        let mut values = [WHITE; 100];

        for (index, value) in values.iter_mut().enumerate() {
            *value = max_value / (100 - index) as f64;
        }

        RgbBrightnessLookupTable { values }
    }

    pub fn get_rgb_from_brightness(&self, brightness: usize) -> &RGB {
        &self.values[brightness]
    }
}
