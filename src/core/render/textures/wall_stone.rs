use super::super::tiling_texture::TilingTexture;

#[allow(unused)]
pub fn load_texture() -> TilingTexture {
    TilingTexture::new_from_bmp_data(include_bytes!("./wall_stone.bmp"))
}
