use crate::render::{rgb::RGB, texture::Texture};
use wasm_bindgen::Clamped;
use web_sys::ImageData;

pub struct ScreenBuffer {
    pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
    center: usize,
}

impl ScreenBuffer {
    pub fn setup(width: usize, height: usize) -> Self {
        ScreenBuffer {
            pixels: vec![255u8; width * height * 4],
            width,
            height,
            center: height/2,
        }
    }

    pub fn _render_solid_colour_column(&mut self, x: &usize, height: usize, colour: &RGB) {
        self.render_column(x, height, &|_wall_pixel_index| {colour})
    }

    pub fn render_textured_column(&mut self, x: &usize, height: usize, texture: &Texture) {
        self.render_column(x, height, &|wall_pixel_index| {
            let texture_y_pos = (wall_pixel_index as f64 / height as f64) * texture.height as f64;
            &texture.get_texel((*x as f64 / 40.0) as usize, texture_y_pos as usize)
        })

    }

    pub fn render_column<'a, F: Fn(usize) -> &'a RGB>(&mut self, x: &usize, mut height: usize, get_colour: &'a F) {
        let mut starting_wall_position = 0;
        if height > self.height {
            starting_wall_position = (height - self.height) / 2;
            height = self.height;
        }

        let half_height = height / 2;
        let top = self.center + half_height;
        let bottom = self.center - half_height;

        const FLOOR_COLOR: RGB = RGB {
            red: 15,
            green: 60,
            blue: 15,
        };
        const SKY_COLOR: RGB = RGB {
            red: 10,
            green: 40,
            blue: 10,
        };

        let pixel_increment = self.width * 4;
        let start_pixel_index = x * 4;
        let bottom_pixel_index = start_pixel_index + (bottom * pixel_increment);
        let top_pixel_index = start_pixel_index + (top * pixel_increment);
        let end_pixel_index = start_pixel_index + (self.height * pixel_increment);
        let mut pixel_index = start_pixel_index;

        while pixel_index < bottom_pixel_index {
            self.pixels[pixel_index] = SKY_COLOR.red;
            self.pixels[pixel_index + 1] = SKY_COLOR.green;
            self.pixels[pixel_index + 2] = SKY_COLOR.blue;
            pixel_index += pixel_increment;
        }
        let mut wall_pixel_index = starting_wall_position;
        while pixel_index < top_pixel_index {
            let colour = get_colour(wall_pixel_index);
            wall_pixel_index += 1;

            self.pixels[pixel_index] = colour.red;
            self.pixels[pixel_index + 1] = colour.green;
            self.pixels[pixel_index + 2] = colour.blue;
            pixel_index += pixel_increment;
        }
        while pixel_index < end_pixel_index {
            self.pixels[pixel_index] = FLOOR_COLOR.red;
            self.pixels[pixel_index + 1] = FLOOR_COLOR.green;
            self.pixels[pixel_index + 2] = FLOOR_COLOR.blue;
            pixel_index += pixel_increment;
        }
    }

    pub fn to_imagedata(&self) -> ImageData {
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.pixels), // Wrap the slice with Clamped
            self.width as u32,
            self.height as u32,
        )
        .expect("couldnt convert screen_buffer to ImageDats")
    }
}
