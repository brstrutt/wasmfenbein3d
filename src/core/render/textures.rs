use super::tiling_texture::TilingTexture;

mod big_floor;
mod floor;
mod wall_stone;
mod wall_wood;

pub struct Textures {
    pub wall_stone: TilingTexture,
    pub wall_wood: TilingTexture,
    pub floor: TilingTexture,
}

impl Textures {
    pub fn load() -> Textures {
        Textures {
            wall_stone: wall_stone::load_texture(),
            wall_wood: wall_wood::load_texture(),
            floor: big_floor::load_texture(),
        }
    }
}
