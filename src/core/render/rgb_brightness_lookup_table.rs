use super::rgb::{RGB, WHITE};

pub const BRIGHTNESS_STEPS: usize = 100;
pub const BRIGHTNESS_STEPS_F64: f64 = BRIGHTNESS_STEPS as f64;
pub const MAX_BRIGHTNESS_INDEX: usize = BRIGHTNESS_STEPS - 1;

#[derive(Clone)]
pub struct RgbBrightnessLookupTable {
    pub values: [RGB; BRIGHTNESS_STEPS],
}

impl RgbBrightnessLookupTable {
    pub fn generate(max_value: &RGB) -> Self {
        let mut values = [WHITE; BRIGHTNESS_STEPS];

        for (index, value) in values.iter_mut().enumerate() {
            let darkness_index = (BRIGHTNESS_STEPS - index) as f64 / 50.0;
            *value = max_value / (1.0 + darkness_index);
        }

        RgbBrightnessLookupTable { values }
    }

    pub fn get_rgb_from_brightness(&self, brightness: usize) -> &RGB {
        &self.values[brightness]
    }
}
