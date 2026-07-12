use std::{cell::RefCell, rc::Rc};

use super::rgb_palette::RgbPalette;
use super::tiling_texture::TilingTexture;

mod big_floor;
mod floor;
mod wall_stone;
mod wall_wood;

pub struct Textures {
    pub wall_stone: Rc<RefCell<TilingTexture>>,
    pub wall_wood: Rc<RefCell<TilingTexture>>,
    pub floor: Rc<RefCell<TilingTexture>>,
}

impl Textures {
    pub fn load(palette: &mut RgbPalette) -> Textures {
        Textures {
            wall_stone: Rc::new(RefCell::new(wall_stone::load_texture(palette))),
            wall_wood: Rc::new(RefCell::new(wall_wood::load_texture(palette))),
            floor: Rc::new(RefCell::new(big_floor::load_texture(palette))),
        }
    }
}
