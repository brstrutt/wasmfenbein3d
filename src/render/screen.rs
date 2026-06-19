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

    pub fn render(&mut self, x: &usize, y: &usize, color: &RGB) {
        let pixel_index = ((y * self.width) + x) * 4;
        self.pixels[pixel_index] = color.red;
        self.pixels[pixel_index + 1] = color.green;
        self.pixels[pixel_index + 2] = color.blue;
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

        for y in 0..bottom {
            self.render(&x, &y, &SKY_COLOR);
        }
        for y in bottom..top {
            self.render(&x, &y, &color);
        }
        for y in top..self.height {
            self.render(&x, &y, &FLOOR_COLOR);
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
