use super::rgb::RGB;
use super::rgb_brightness_lookup_table::RgbBrightnessLookupTable;
use std::collections::HashMap;

pub type RgbPalette = HashMap<RGB, RgbBrightnessLookupTable>;
