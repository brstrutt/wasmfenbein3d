use crate::render::{texture::Texture, textures::wall::load_wall_texture};

mod wall;

pub struct Textures {
    pub wall: Texture,
}

impl Textures {
    pub fn load() -> Textures {
        Textures {
            wall: load_wall_texture(),
        }
    }
}
