use std::{cell::RefCell, rc::Rc};

use crate::core::render::tiling_texture::TilingTexture;

#[derive(Clone)]
pub struct Painting {
    pub texture: Rc<RefCell<TilingTexture>>,
}
