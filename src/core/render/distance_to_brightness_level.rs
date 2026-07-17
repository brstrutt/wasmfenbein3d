use super::rgb_brightness_lookup_table::BRIGHTNESS_STEPS_F64;

pub fn distance_to_brightness_level(distance: f64) -> usize {
    (BRIGHTNESS_STEPS_F64 - (distance * 8.0)) as usize
}
