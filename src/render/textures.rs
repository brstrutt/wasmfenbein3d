use crate::render::texture::Texture;

mod floor;
mod wall;

pub struct Textures {
    pub wall: Texture,
    pub floor: Texture,
}

impl Textures {
    pub fn load() -> Textures {
        Textures {
            wall: wall::load_texture(),
            floor: floor::load_texture(),
        }
    }
}
