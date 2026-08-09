use super::{
    colour::Colour, rgb_palette::RgbPalette, texel_provider::TexelProvider, texture::Texture,
};
use crate::core::primitives::{line2d::Line2D, point2d::Point2D};

#[derive(Clone)]
pub struct TilingTexture {
    pub texture: Texture,
    pub size_bitwise_mask: usize,
    pub size_bitwise_mask_i: isize,
}

impl TilingTexture {
    pub fn new_from_bmp_data(bmp_data: &[u8], palette: &mut RgbPalette) -> Self {
        let texture = Texture::new_from_bmp_data(bmp_data, palette);

        if texture.width != texture.height {
            panic!(
                "Couldn't load tiling texture. The texture is not a perfect square (size is {}X{})",
                texture.width, texture.height
            );
        }

        let size_is_power_of_two = texture.width.count_ones() == 1;
        if !size_is_power_of_two {
            panic!(
                "Couldn't load tiling texture. Width/Height are not powers of two (size is {}X{})",
                texture.width, texture.height
            );
        }

        let bitwise_mask = texture.width - 1;
        TilingTexture {
            texture,
            size_bitwise_mask: bitwise_mask,
            size_bitwise_mask_i: bitwise_mask as isize,
        }
    }

    pub fn get_texel(&self, x: isize, y: isize) -> &dyn Colour {
        let x = x & self.size_bitwise_mask_i;
        let y = y & self.size_bitwise_mask_i;
        self.texture.get_texel(x, y)
    }

    pub fn get_texel_column_on_line_with_scale(
        &self,
        line: &Line2D,
        point: &Point2D,
        scale: f64,
    ) -> usize {
        self.texture
            .get_texel_column_on_line_with_scale(line, point, scale)
    }

    pub fn width(&self) -> usize {
        self.texture.width
    }
    pub fn width_f64(&self) -> &f64 {
        &self.texture.width_f64
    }

    pub fn height(&self) -> usize {
        self.texture.height
    }
    pub fn height_f64(&self) -> &f64 {
        &self.texture.height_f64
    }
}

impl TexelProvider for TilingTexture {
    fn get_texel(&self, x: isize, y: isize) -> &dyn Colour {
        let x = x & self.size_bitwise_mask_i;
        let y = y & self.size_bitwise_mask_i;
        self.texture.get_texel(x, y)
    }

    fn width(&self) -> usize {
        self.texture.width
    }
    fn width_f64(&self) -> &f64 {
        &self.texture.width_f64
    }

    fn height(&self) -> usize {
        self.texture.height
    }
    fn height_f64(&self) -> &f64 {
        &self.texture.height_f64
    }
}
