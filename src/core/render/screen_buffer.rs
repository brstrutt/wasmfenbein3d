use super::{rgb::RGB, tiling_texture::TilingTexture};
use crate::core::world::camera::Camera;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

pub struct ScreenBuffer {
    pixels: Vec<u8>,
    already_drawn: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

impl ScreenBuffer {
    pub fn setup(width: usize, height: usize) -> Self {
        ScreenBuffer {
            pixels: vec![255u8; width * height * 4],
            already_drawn: vec![false; width * height],
            width,
            height,
        }
    }

    pub fn reset_draw_history(&mut self) {
        self.already_drawn.fill(false)
    }

    pub fn render_pixel(&mut self, pixel_index: usize, colour: &RGB) {
        self.pixels[pixel_index << 2] = colour.red;
        self.pixels[(pixel_index << 2) + 1] = colour.green;
        self.pixels[(pixel_index << 2) + 2] = colour.blue;
        self.already_drawn[pixel_index] = true;
    }

    pub fn render_textured_row(
        &mut self,
        y: &usize,
        camera: &Camera,
        dist_to_floor: f64,
        texture: &TilingTexture,
        brightness: usize,
    ) {
        let rgb_pixel_increment = 4;
        let row_length = self.width * 4;

        let mut rgb_pixel_index = y * row_length;
        let end_point = rgb_pixel_index + row_length;

        let mut x = 0;
        let mut pixel_index = y * self.width;
        while rgb_pixel_index < end_point {
            if !self.already_drawn[pixel_index] {
                let ray = camera.ray_for_column(x);
                let position = ray.origin + (ray.direction * dist_to_floor);
                let colour = texture
                    .get_texel((position.x * 16.0) as isize, (position.y * 16.0) as isize)
                    .at_brightness(brightness);

                self.pixels[rgb_pixel_index] = colour.red;
                self.pixels[rgb_pixel_index + 1] = colour.green;
                self.pixels[rgb_pixel_index + 2] = colour.blue;
            }

            rgb_pixel_index += rgb_pixel_increment;
            x += 1;
            pixel_index += 1;
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
