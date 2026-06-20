use crate::render::rgb::RGB;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

pub struct ScreenBuffer {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
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

    pub fn render_column(&mut self, x: &usize, mut height: usize, color: &RGB) {
        if height > self.height {
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
        while pixel_index < top_pixel_index {
            self.pixels[pixel_index] = color.red;
            self.pixels[pixel_index + 1] = color.green;
            self.pixels[pixel_index + 2] = color.blue;
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
