use std::{cell::RefCell, rc::Rc};

use crate::core::{primitives::line2d::Line2D, render::tiling_texture::TilingTexture};

#[derive(Clone)]
pub struct Wall {
    pub position: Line2D,
    pub texture: Rc<RefCell<TilingTexture>>,
}

impl Wall {
    pub fn new(line: Line2D, texture: &Rc<RefCell<TilingTexture>>) -> Self {
        Wall {
            position: line,
            texture: texture.clone(),
        }
    }
}
