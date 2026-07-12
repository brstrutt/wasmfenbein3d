use super::rgb::RGB;
use super::rgb_brightness_lookup_table::RgbBrightnessLookupTable;
use super::rgb_palette::RgbPalette;

#[derive(Clone)]
pub struct RGBV {
    pub base: RGB,
    pub brightness_variants: RgbBrightnessLookupTable,
}

impl RGBV {
    pub fn from_rgb(base: &RGB, palette: &mut RgbPalette) -> Self {
        if !palette.contains_key(base) {
            palette.insert(base.clone(), RgbBrightnessLookupTable::generate(base));
        }
        let brightness_variants = palette
            .get(base)
            .expect("Inserting a gradiant into the lookup")
            .clone();

        RGBV {
            base: base.clone(),
            brightness_variants,
        }
    }

    pub fn from_u8(rgb_bytes: &[u8; 3], palette: &mut RgbPalette) -> Self {
        RGBV::from_rgb(&RGB::from_u8(rgb_bytes), palette)
    }

    pub fn at_brightness(&self, brightness: usize) -> &RGB {
        &self.brightness_variants.get_rgb_from_brightness(brightness)
    }
}
