use super::rgb::{RGB, WHITE};

pub const BRIGHTNESS_STEPS: usize = 20;
pub const BRIGHTNESS_STEPS_F64: f64 = BRIGHTNESS_STEPS as f64;
pub const MAX_BRIGHTNESS_INDEX: usize = BRIGHTNESS_STEPS - 1;
const DARKNESS_SCALE_FACTOR: f64 = 5.0 / BRIGHTNESS_STEPS_F64;

#[derive(Clone)]
pub struct RgbBrightnessLookupTable {
    pub values: [RGB; BRIGHTNESS_STEPS],
}

impl RgbBrightnessLookupTable {
    pub fn generate(max_value: &RGB) -> Self {
        let mut values = [WHITE; BRIGHTNESS_STEPS];

        for (index, value) in values.iter_mut().enumerate() {
            let lightness_index = BRIGHTNESS_STEPS_F64 - index as f64;
            *value = max_value / f64::max(DARKNESS_SCALE_FACTOR * lightness_index, 1.0);
        }

        RgbBrightnessLookupTable { values }
    }

    pub fn get_rgb_from_brightness(&self, brightness: usize) -> &RGB {
        &self.values[brightness]
    }
}
