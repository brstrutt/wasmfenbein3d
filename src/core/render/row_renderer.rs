use crate::core::world::camera::Camera;

use super::{screen_buffer::ScreenBuffer, tiling_texture::TilingTexture};

pub fn render_row(
    y: &usize,
    camera: &Camera,
    dist_to_floor: f64,
    texture: &TilingTexture,
    brightness: usize,
    screen_buffer: &mut ScreenBuffer,
) {
    let rgb_pixel_increment = 1;
    let row_length = screen_buffer.width;

    let mut rgb_pixel_index = y * row_length;
    let end_point = rgb_pixel_index + row_length;

    let mut x = 0;
    let mut pixel_index = y * screen_buffer.width;
    while rgb_pixel_index < end_point {
        if !screen_buffer.pixel_drawn(pixel_index) {
            let ray = camera.ray_for_column(x);
            let position = ray.origin + (ray.direction * dist_to_floor);
            let colour = texture
                .get_texel((position.x * 16.0) as isize, (position.y * 16.0) as isize)
                .at_brightness(brightness);

            screen_buffer.render_pixel(rgb_pixel_index, colour);
        }

        rgb_pixel_index += rgb_pixel_increment;
        x += 1;
        pixel_index += 1;
    }
}
