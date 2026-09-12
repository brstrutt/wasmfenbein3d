use std::cmp::min;

use crate::core::render::rgb_brightness_lookup_table::MAX_BRIGHTNESS_INDEX;

use super::rgb_brightness_lookup_table::BRIGHTNESS_STEPS_F64;

const BRIGHTNESS_SCALING: f64 = 10.0 / BRIGHTNESS_STEPS_F64;

pub fn distance_to_brightness_level(distance: f64) -> usize {
    MAX_BRIGHTNESS_INDEX
        - min(
            (distance / BRIGHTNESS_SCALING) as usize,
            MAX_BRIGHTNESS_INDEX,
        )
}
