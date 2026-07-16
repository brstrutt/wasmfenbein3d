use std::rc::Rc;
use wasmfenbein3d::core::render::{rgb_palette::RgbPalette, tiling_texture::TilingTexture};

#[allow(unused)]
pub fn load_texture(palette: &mut RgbPalette) -> Rc<TilingTexture> {
    Rc::new(TilingTexture::new_from_bmp_data(
        include_bytes!("./big_floor.bmp"),
        palette,
    ))
}
