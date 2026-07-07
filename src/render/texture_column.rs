use crate::render::{rgb::RGB, texture::Texture};
use std::ops;

pub struct TextureColumn {
    texture: Texture,
}

impl TextureColumn {
    pub fn from_texture(source_texture: &Texture, column: usize) -> Self {
        let mut texels_slice = vec![RGB::white(); source_texture.height];
        for (i, sub_texel) in texels_slice.iter_mut().enumerate() {
            *sub_texel = source_texture.texels[column + (i * source_texture.width)].clone();
        }
        TextureColumn {
            texture: Texture {
                width: 1,
                height: source_texture.height,
                texels: texels_slice,
            },
        }
    }

    pub fn get_texel(&self, y: isize) -> &RGB {
        &self.texture.get_texel(0, y)
    }
}

impl ops::Div<f64> for &TextureColumn {
    type Output = TextureColumn;

    fn div(self, rhs: f64) -> TextureColumn {
        TextureColumn {
            texture: &self.texture / rhs,
        }
    }
}
