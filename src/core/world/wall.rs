use std::{cell::RefCell, rc::Rc};

use super::painting::Painting;
use crate::core::{primitives::line2d::Line2D, render::tiling_texture::TilingTexture};

pub const WALL_HEIGHT: f64 = 2.0;

#[derive(Clone)]
pub struct Wall {
    pub position: Line2D,
    pub texture: Rc<RefCell<TilingTexture>>,
    pub painting: Painting,
}

impl Wall {
    pub fn new(line: Line2D, texture: &Rc<RefCell<TilingTexture>>, painting: Painting) -> Self {
        Wall {
            position: line,
            texture: texture.clone(),
            painting,
        }
    }

    pub fn get_texture_at_point(
        &self,
        wall_x_pos: isize,
        _wall_y_pos: isize,
    ) -> &Rc<RefCell<TilingTexture>> {
        if wall_x_pos < self.painting.texture.borrow().texture.width as isize {
            &self.painting.texture
        } else {
            &self.texture
        }
    }
}
