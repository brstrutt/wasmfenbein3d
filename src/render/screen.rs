use crate::{
    render::{
        rgb::RGB,
        texture::{TEXTURE_SIZE_BITS, Texture},
    },
    world::{
        camera::Camera,
        walls::{WALL_HEIGHT, WallCollision},
    },
};
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

    pub fn render_solid_colour_column(
        &mut self,
        x: &usize,
        height: f64,
        colour: &RGB,
        colour_adjustment: f64,
    ) {
        let colour = colour.clone() / colour_adjustment;
        self.render_column(x, height, &|_wall_pixel_index| &colour)
    }

    pub fn render_textured_column(
        &mut self,
        x: &usize,
        height: f64,
        texture: &Texture,
        wall_details: &WallCollision,
        colour_adjustment: f64,
    ) {
        let texture_x_pos = texture.get_texel_column_on_line_with_scale(
            &wall_details.wall,
            &wall_details.intersection,
            1.0,
        );
        let mut texture = texture.get_texel_column(texture_x_pos);
        texture = &texture / colour_adjustment;

        self.render_column(x, height, &|wall_pixel_index| {
            let texture_y_pos = (wall_pixel_index as f64 / height) * texture.height as f64;
            &texture.texels[(texture_y_pos * WALL_HEIGHT) as usize & TEXTURE_SIZE_BITS]
        })
    }

    fn render_column<'a, F: Fn(usize) -> &'a RGB>(
        &mut self,
        x: &usize,
        height: f64,
        get_colour: &'a F,
    ) {
        let mut starting_wall_position = 0;
        let mut height = height as usize;
        if height > self.height {
            starting_wall_position = (height - self.height) / 2;
            height = self.height;
        }

        let half_height = height / 2;
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
            let colour = get_colour(wall_pixel_index).clone();

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
        texture: &Texture,
        colour_adjustment: f64,
    ) {
        let adjusted_texture = texture / colour_adjustment;
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
                let colour = adjusted_texture
                    .get_texel((position.x * 16.0) as isize, (position.y * 16.0) as isize);

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
