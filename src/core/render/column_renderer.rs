use std::cell::RefMut;

use super::{
    column_data::ColumnData, distance_to_brightness_level::distance_to_brightness_level,
    screen_buffer::ScreenBuffer, texel_provider::TexelProvider,
};
use crate::core::world::wall::WALL_HEIGHT;

pub struct ColumnRenderer<'a> {
    screen: ScreenSpace,
    brightness_level: usize,
    render_plan: Vec<ColumnSegment<'a>>,
}

struct ScreenSpace {
    current_pixel_index: usize,
    column_last_pixel_index: usize,
}

struct ColumnSegment<'a> {
    texture: &'a dyn TexelProvider,
    screen_space_end_y: usize,
    texture_space_x: f64,
    texture_space_start_y: f64,
    texture_space_y_increment: f64,
}

impl<'a> ColumnRenderer<'a> {
    pub fn init<Screen: ScreenBuffer>(
        screen_x: &'a usize,
        screen_height: &'a f64,
        column: &'a ColumnData<'a>,
        screen_buffer: &RefMut<Screen>,
    ) -> Self {
        let wall_space_pixel_height = WALL_HEIGHT / column.height_pixels;
        let mut wall_space_pixel_increment = wall_space_pixel_height;
        let mut screen_space_wall_increment = column.height_pixels / WALL_HEIGHT;

        let mut wall_start_y = 0.0;

        let mut screen_start_y: isize = 0;
        let mut screen_end_y: isize = *screen_height as isize;

        if column.height_pixels > *screen_height {
            let offscreen_pixel_count = column.height_pixels - screen_height;
            let half_offscreen_pixel_count = offscreen_pixel_count / 2.0;
            let wallspace_adjustment = half_offscreen_pixel_count * wall_space_pixel_height;
            wall_start_y += wallspace_adjustment;
        } else {
            // Ceil() the result to make the resulting wall height smaller. This avoids an issue where the wall is one pixel too tall and triggers index out of bounds errors when accessing the texture array
            let pixels_from_edge = ((screen_height - column.height_pixels) / 2.0).ceil() as isize;
            screen_start_y += pixels_from_edge;
            screen_end_y -= pixels_from_edge;

            //Update wall_space_pixel_increment as the start/end positions may not be exactly column_height distance apart
            wall_space_pixel_increment = WALL_HEIGHT / (screen_end_y - screen_start_y) as f64;
            screen_space_wall_increment = (screen_end_y - screen_start_y) as f64 / WALL_HEIGHT;
        }

        let wall_texture = column.nearest_wall_intersection.wall.texture.as_ref();
        let wall_texture_space_x = column.wall_x_pos * wall_texture.width_f64();
        let wall_texture_space_y_increment = wall_space_pixel_increment * wall_texture.height_f64();

        let mut segment_start_y: isize = 0;
        let wall_space_y_to_screen_space_y = |wall_space_y: f64| {
            ((wall_space_y - wall_start_y) * screen_space_wall_increment).round() as isize
        };
        let screen_space_y_to_wall_space_y = |screen_space_y: isize| {
            (screen_space_y as f64 / screen_space_wall_increment) + wall_start_y
        };

        let create_wall_segment = |screen_space_start, screen_space_end_y| ColumnSegment {
            texture: wall_texture,
            screen_space_end_y: screen_space_end_y as usize,
            texture_space_x: wall_texture_space_x,
            texture_space_start_y: screen_space_y_to_wall_space_y(screen_space_start)
                * wall_texture.height_f64(),
            texture_space_y_increment: wall_texture_space_y_increment,
        };

        let mut render_plan = vec![];
        for painting in &column.paintings {
            let painting_top_screen_space =
                wall_space_y_to_screen_space_y(painting.top_left_corner.y);
            let painting_bottom_screen_space =
                wall_space_y_to_screen_space_y(painting.bottom_right_corner.y);

            let texture_space_y_increment = painting.texture.height_f64()
                / (painting_bottom_screen_space - painting_top_screen_space) as f64;

            let texture_space_start_y = if painting_top_screen_space > segment_start_y {
                render_plan.push(create_wall_segment(
                    segment_start_y.clone(),
                    painting_top_screen_space,
                ));
                segment_start_y = painting_top_screen_space;
                0.0
            } else {
                (painting_top_screen_space as f64 * -1.0) * texture_space_y_increment
            };

            if painting_bottom_screen_space > segment_start_y {
                let screen_space_end_y = painting_bottom_screen_space.min(screen_end_y as isize);

                render_plan.push(ColumnSegment {
                    texture: painting.texture.as_ref(),
                    screen_space_end_y: painting_bottom_screen_space as usize,
                    texture_space_x: (column.wall_x_pos - painting.top_left_corner.x)
                        * painting.texture.width_f64()
                        / painting.width,
                    texture_space_start_y: texture_space_start_y,
                    texture_space_y_increment,
                });
                segment_start_y = screen_space_end_y;
            }
        }

        if segment_start_y < screen_end_y {
            render_plan.push(create_wall_segment(segment_start_y, screen_end_y));
        }

        ColumnRenderer {
            screen: ScreenSpace {
                current_pixel_index: screen_buffer
                    .coord_to_pixel_index(screen_x, &(screen_start_y as usize)),
                column_last_pixel_index: screen_buffer
                    .coord_to_pixel_index(screen_x, &(screen_end_y as usize)),
            },
            brightness_level: distance_to_brightness_level(column.distance_from_camera),
            render_plan,
        }
    }

    pub fn render_column<Screen: ScreenBuffer>(&mut self, screen_buffer: &mut RefMut<Screen>) {
        let mut screen_space_y = 0;
        for segment in &self.render_plan {
            let mut tex_space_y = segment.texture_space_start_y;
            while screen_space_y < segment.screen_space_end_y
                && self.screen.current_pixel_index < self.screen.column_last_pixel_index
            {
                let colour = segment
                    .texture
                    .get_texel(segment.texture_space_x as isize, tex_space_y as isize)
                    .at_brightness_as_rgb(self.brightness_level);

                screen_buffer.render_pixel(self.screen.current_pixel_index, colour);
                screen_buffer.mark_pixel_as_rendered(self.screen.current_pixel_index);

                self.screen.current_pixel_index += screen_buffer.column_pixel_index_increment();
                tex_space_y += segment.texture_space_y_increment;
                screen_space_y += 1;
            }
        }
    }
}
