use crate::render::texture::Texture;

pub fn load_wall_texture() -> Texture {
    Texture::new_from_bmp_data(include_bytes!("./wall.bmp"))
}
