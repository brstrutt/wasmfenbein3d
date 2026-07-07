use crate::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::{rgb::RGB, texture::Texture},
};
use std::ops;

pub const TEXTURE_SIZE: usize = 16;
pub const TEXTURE_SIZE_BITS: usize = TEXTURE_SIZE - 1;
pub const TEXTURE_SIZE_BITS_I: isize = TEXTURE_SIZE_BITS as isize;

pub struct TilingTexture {
    pub texture: Texture,
}

impl TilingTexture {
    pub fn new_from_bmp_data(bmp_data: &[u8]) -> Self {
        let texture = Texture::new_from_bmp_data(bmp_data);

        if texture.width != TEXTURE_SIZE || texture.height != TEXTURE_SIZE {
            panic!(
                "Couldn't load tiling texture. It was not {}X{}",
                TEXTURE_SIZE, TEXTURE_SIZE
            );
        }

        TilingTexture { texture }
    }

    pub fn get_texel(&self, x: isize, y: isize) -> &RGB {
        let x = x & TEXTURE_SIZE_BITS_I;
        let y = y & TEXTURE_SIZE_BITS_I;
        &self.texture.get_texel(x, y)
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

    pub fn height(&self) -> usize {
        self.texture.height
    }
}

impl ops::Div<f64> for &TilingTexture {
    type Output = TilingTexture;

    fn div(self, rhs: f64) -> TilingTexture {
        TilingTexture {
            texture: &self.texture / rhs,
        }
    }
}
