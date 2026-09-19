use super::rgb::RGB;
use super::rgb_brightness_lookup_table::RgbBrightnessLookupTable;
use std::collections::HashMap;
use std::rc::Rc;

pub type RgbPalette = HashMap<RGB, Rc<RgbBrightnessLookupTable>>;
