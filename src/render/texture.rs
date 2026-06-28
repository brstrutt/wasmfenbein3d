use crate::render::rgb::RGB;

pub struct Texture {
    texels: Vec<RGB>,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn get_default() -> Self {
        const LIGHT_GREEN: RGB = RGB {
            red: 60,
            green: 175,
            blue: 30,
        };

        const DARK_GREEN: RGB = RGB {
            red: 30,
            green: 100,
            blue: 60,
        };

        Texture {
            width: 4,
            height: 4,
            texels: vec![
                LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN, DARK_GREEN,
                DARK_GREEN, LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN,
                LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN, DARK_GREEN,
                DARK_GREEN, LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN,
            ],
        }
    }

    pub fn get_texel<'a>(&'a self, x: usize, y: usize) -> &'a RGB {
        let x = x % self.width;
        let y = y % self.height;
        &self.texels[y * self.width + x]
    }
}
