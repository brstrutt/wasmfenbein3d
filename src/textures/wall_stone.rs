use std::{cell::RefCell, rc::Rc};
use wasmfenbein3d::core::render::{rgb_palette::RgbPalette, tiling_texture::TilingTexture};

#[allow(unused)]
pub fn load_texture(palette: &mut RgbPalette) -> Rc<RefCell<TilingTexture>> {
    Rc::new(RefCell::new(TilingTexture::new_from_bmp_data(
        include_bytes!("./wall_stone.bmp"),
        palette,
    )))
}
