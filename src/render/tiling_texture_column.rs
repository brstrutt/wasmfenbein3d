use crate::render::{
    rgb::RGB,
    texture_column::TextureColumn,
    tiling_texture::{TEXTURE_SIZE_BITS, TEXTURE_SIZE_BITS_I, TilingTexture},
};
use std::ops;

pub struct TilingTextureColumn {
    texture_column: TextureColumn,
}

impl TilingTextureColumn {
    pub fn from_texture(source_texture: &TilingTexture, column: usize) -> Self {
        let column = column & TEXTURE_SIZE_BITS;
        TilingTextureColumn {
            texture_column: TextureColumn::from_texture(&source_texture.texture, column),
        }
    }

    pub fn get_texel(&self, y: isize) -> &RGB {
        let y = y & TEXTURE_SIZE_BITS_I;
        &self.texture_column.get_texel(y)
    }
}

impl ops::Div<f64> for &TilingTextureColumn {
    type Output = TilingTextureColumn;

    fn div(self, rhs: f64) -> TilingTextureColumn {
        TilingTextureColumn {
            texture_column: &self.texture_column / rhs,
        }
    }
}
