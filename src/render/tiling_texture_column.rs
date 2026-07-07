use crate::render::{rgb::RGB, texture_column::TextureColumn, tiling_texture::TilingTexture};
use std::ops;

pub struct TilingTextureColumn {
    texture_column: TextureColumn,
    size_bitwise_mask_i: isize,
}

impl TilingTextureColumn {
    pub fn from_texture(source_texture: &TilingTexture, column: usize) -> Self {
        let column = column & source_texture.size_bitwise_mask;
        TilingTextureColumn {
            texture_column: TextureColumn::from_texture(&source_texture.texture, column),
            size_bitwise_mask_i: source_texture.size_bitwise_mask as isize,
        }
    }

    pub fn get_texel(&self, y: isize) -> &RGB {
        let y = y & self.size_bitwise_mask_i;
        &self.texture_column.get_texel(y)
    }
}

impl ops::Div<f64> for &TilingTextureColumn {
    type Output = TilingTextureColumn;

    fn div(self, rhs: f64) -> TilingTextureColumn {
        TilingTextureColumn {
            texture_column: &self.texture_column / rhs,
            size_bitwise_mask_i: self.size_bitwise_mask_i,
        }
    }
}
