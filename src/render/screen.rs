use crate::{
    primitives::point2d::Point2D,
    render::{rgb::RGB, texture::Texture},
    world::walls::{WALL_HEIGHT, WallCollision},
};
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
            center: height / 2,
        }
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
            WALL_HEIGHT,
        );
        let mut texture = texture.get_texel_column(texture_x_pos);
        texture = &texture / colour_adjustment;

        self.render_column(x, height, &|wall_pixel_index| {
            let texture_y_pos = (wall_pixel_index as f64 / height) * texture.height as f64;
            texture.get_texel(texture_x_pos, texture_y_pos as usize)
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

        let pixel_increment = self.width * 4;
        let start_pixel_index = x * 4;
        let bottom_pixel_index = start_pixel_index + (bottom * pixel_increment);
        let top_pixel_index = start_pixel_index + (top * pixel_increment);
        let mut pixel_index = bottom_pixel_index;

        let mut wall_pixel_index = starting_wall_position;
        while pixel_index < top_pixel_index {
            let colour = get_colour(wall_pixel_index).clone();
            wall_pixel_index += 1;

            self.pixels[pixel_index] = colour.red;
            self.pixels[pixel_index + 1] = colour.green;
            self.pixels[pixel_index + 2] = colour.blue;
            pixel_index += pixel_increment;
        }
    }

    pub fn render_textured_row(
        &mut self,
        y: &usize,
        camera_position: Point2D,
        left_ray_dir: Point2D,
        right_ray_dir: Point2D,
        screen_width_pixels: f64,
        dist_to_floor: f64,
        texture: &Texture,
    ) {
        let floor_position_increment =
            (right_ray_dir - left_ray_dir) * (dist_to_floor / screen_width_pixels);

        let initial_floor_position = camera_position + (left_ray_dir * dist_to_floor);

        let pixel_increment = 4;
        let row_length = self.width * 4;

        let mut pixel_index = y * row_length;
        let end_point = pixel_index + row_length;
        let mut current_floor_position = initial_floor_position;

        while pixel_index < end_point {
            let colour = texture
                .get_texel(
                    (current_floor_position.x * 10.0) as usize,
                    (current_floor_position.y * 10.0) as usize,
                )
                .clone();

            self.pixels[pixel_index] = colour.red;
            self.pixels[pixel_index + 1] = colour.green;
            self.pixels[pixel_index + 2] = colour.blue;
            pixel_index += pixel_increment;
            current_floor_position += floor_position_increment;
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
