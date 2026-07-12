use super::super::rgb_palette::RgbPalette;
use super::super::tiling_texture::TilingTexture;

#[allow(unused)]
pub fn load_texture(palette: &mut RgbPalette) -> TilingTexture {
    TilingTexture::new_from_bmp_data(include_bytes!("./wall_wood.bmp"), palette)
}
