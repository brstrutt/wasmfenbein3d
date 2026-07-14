use super::tiling_texture::TilingTexture;
use crate::core::world::{camera::Camera, wall::WALL_HEIGHT, walls::WallCollision};
use wasm_bindgen::Clamped;
use web_sys::ImageData;

pub struct ScreenBuffer {
    pixels: Vec<u8>,
    already_drawn: Vec<bool>,
    pub width: usize,
    pub height: usize,
    center: usize,
}

impl ScreenBuffer {
    pub fn setup(width: usize, height: usize) -> Self {
        ScreenBuffer {
            pixels: vec![255u8; width * height * 4],
            already_drawn: vec![false; width * height],
            width,
            height,
            center: height / 2,
        }
    }

    pub fn reset_draw_history(&mut self) {
        self.already_drawn.fill(false)
    }

    pub fn render_textured_column(
        &mut self,
        x: &usize,
        height: f64,
        wall_details: &WallCollision,
        brightness: usize,
    ) {
        let texture = wall_details.wall.texture.borrow();
        let wall_x_pos = texture.get_texel_column_on_line_with_scale(
            &wall_details.wall.position,
            &wall_details.intersection,
            1.0,
        ) as isize;

        let mut starting_wall_position = 0;
        let mut height_usize = height as usize;
        if height_usize > self.height {
            starting_wall_position = (height_usize - self.height) / 2;
            height_usize = self.height;
        }

        let half_height = height_usize / 2;
        let top = self.center + half_height;
        let bottom = self.center - half_height;

        let pixel_increment = self.width;
        let rgb_pixel_increment = pixel_increment * 4;
        let start_rgb_pixel_index = x * 4;
        let bottom_rgb_pixel_index = start_rgb_pixel_index + (bottom * rgb_pixel_increment);
        let top_rgb_pixel_index = start_rgb_pixel_index + (top * rgb_pixel_increment);
        let mut rgb_pixel_index = bottom_rgb_pixel_index;

        let mut wall_pixel_index = starting_wall_position;
        let mut pixel_index = x + (bottom * pixel_increment);
        while rgb_pixel_index < top_rgb_pixel_index {
            let wall_y_pos =
                ((wall_pixel_index as f64 / height) * texture.height() as f64) as isize;

            let texture = wall_details
                .wall
                .get_texture_at_point(wall_x_pos, wall_y_pos)
                .borrow();
            let texel = texture.get_texel(wall_x_pos, wall_y_pos as isize * WALL_HEIGHT as isize);
            let colour = texel.at_brightness(brightness);

            self.pixels[rgb_pixel_index] = colour.red;
            self.pixels[rgb_pixel_index + 1] = colour.green;
            self.pixels[rgb_pixel_index + 2] = colour.blue;
            self.already_drawn[pixel_index] = true;

            wall_pixel_index += 1;
            rgb_pixel_index += rgb_pixel_increment;
            pixel_index += pixel_increment;
        }
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
