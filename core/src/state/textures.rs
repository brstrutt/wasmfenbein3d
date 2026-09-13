use std::{collections::HashMap, rc::Rc};

use crate::render::{
    rgb_palette::RgbPalette, texel_provider::TexelProvider, texture::Texture,
    tiling_texture::TilingTexture,
};

pub struct RawTexture {
    pub id: &'static str,
    pub bytes: &'static [u8],
}

#[macro_export]
macro_rules! include_texture {
    ($name:literal) => {
        RawTexture {
            id: $name,
            bytes: include_bytes!(concat!("./textures/", $name, ".bmp")),
        }
    };
}

pub struct TextureLibrary {
    textures: HashMap<String, Rc<Box<dyn TexelProvider>>>,
    palette: RgbPalette,
}

impl TextureLibrary {
    pub fn new() -> Self {
        TextureLibrary {
            textures: HashMap::new(),
            palette: RgbPalette::new(),
        }
    }

    pub fn load(tiling_textures: &[&RawTexture], textures: &[&RawTexture]) -> Self {
        let mut library = TextureLibrary::new();
        for raw_texture in tiling_textures {
            library.insert_tiling_texture(&raw_texture);
        }
        for raw_texture in textures {
            library.insert_texture(&raw_texture);
        }
        library
    }

    pub fn insert_tiling_texture(&mut self, raw_data: &RawTexture) {
        self.textures.insert(
            String::from(raw_data.id),
            Rc::new(Box::new(TilingTexture::new_from_bmp_data(
                raw_data.bytes,
                &mut self.palette,
            ))),
        );
    }
    pub fn insert_texture(&mut self, raw_data: &RawTexture) {
        self.textures.insert(
            String::from(raw_data.id),
            Rc::new(Box::new(Texture::new_from_bmp_data(
                raw_data.bytes,
                &mut self.palette,
            ))),
        );
    }

    pub fn get(&self, id: &str) -> &Rc<Box<dyn TexelProvider>> {
        self.textures.get(id).unwrap()
    }
}
