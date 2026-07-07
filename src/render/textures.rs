use crate::render::tiling_texture::TilingTexture;

mod big_floor;
mod floor;
mod wall;

pub struct Textures {
    pub wall: TilingTexture,
    pub floor: TilingTexture,
}

impl Textures {
    pub fn load() -> Textures {
        Textures {
            wall: wall::load_texture(),
            floor: big_floor::load_texture(),
        }
    }
}
