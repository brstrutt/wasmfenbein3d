use std::cmp::min;

use crate::render::rgb_brightness_lookup_table::MAX_BRIGHTNESS_INDEX;

use super::rgb_brightness_lookup_table::BRIGHTNESS_STEPS_F64;

pub fn distance_to_brightness_level(distance: f64) -> usize {
    MAX_BRIGHTNESS_INDEX
        - min(
            (distance * BRIGHTNESS_STEPS_F64) as usize >> 3,
            MAX_BRIGHTNESS_INDEX,
        )
}
