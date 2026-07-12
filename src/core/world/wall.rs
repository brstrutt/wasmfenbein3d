use crate::core::{primitives::line2d::Line2D, render::tiling_texture::TilingTexture};

#[derive(Clone)]
pub struct Wall {
    pub position: Line2D,
    pub texture: TilingTexture,
}

impl Wall {
    pub fn new(line: Line2D, texture: &TilingTexture) -> Self {
        Wall {
            position: line,
            texture: texture.clone(),
        }
    }
}
