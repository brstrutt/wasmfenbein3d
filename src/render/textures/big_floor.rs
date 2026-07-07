use crate::render::tiling_texture::TilingTexture;

pub fn load_texture() -> TilingTexture {
    TilingTexture::new_from_bmp_data(include_bytes!("./big_floor.bmp"))
}
