use std::{cell::RefCell, rc::Rc};

use crate::core::render::texture::Texture;

#[derive(Clone)]
pub struct Painting {
    pub texture: Rc<RefCell<Texture>>,
}
