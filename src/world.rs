use wasmfenbein3d::core::state::{
    textures::TextureLibrary,
    world::{load_from_json::load_walls_from_json, wall::Wall},
};

pub fn load_walls(textures: &TextureLibrary) -> Vec<Wall> {
    load_walls_from_json(textures, include_str!("./world/data.json"))
}
