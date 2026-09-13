use std::rc::Rc;

use super::colour::Colour;
use super::rgb::RGB;
use super::rgb_brightness_lookup_table::RgbBrightnessLookupTable;
use super::rgb_palette::RgbPalette;

#[derive(Clone)]
pub struct RGBV {
    pub base: RGB,
    pub brightness_variants: Rc<RgbBrightnessLookupTable>,
}

impl RGBV {
    pub fn from_rgb(base: &RGB, palette: &mut RgbPalette) -> Self {
        RGBV {
            base: base.clone(),
            brightness_variants: palette
                .entry(base.clone())
                .or_insert(Rc::new(RgbBrightnessLookupTable::generate(base)))
                .clone(),
        }
    }

    pub fn from_u8(rgb_bytes: &[u8; 3], palette: &mut RgbPalette) -> Self {
        RGBV::from_rgb(&RGB::from_u8(rgb_bytes), palette)
    }
}

impl Colour for RGBV {
    fn at_brightness_as_rgb(&self, brightness: usize) -> &RGB {
        self.brightness_variants.get_rgb_from_brightness(brightness)
    }
}
