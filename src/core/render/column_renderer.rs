use super::{
    distance_to_brightness_level::distance_to_brightness_level, screen_buffer::ScreenBuffer,
};
use crate::core::{
    primitives::point2d::Point2D,
    world::{wall::WALL_HEIGHT, walls::WallCollision},
};

pub struct ColumnRenderer<'a> {
    screen_x: &'a usize,
    screen_width: &'a usize,
    screen_height: &'a usize,
    screen_half_height: &'a usize,
    nearest_wall_intersection: &'a WallCollision<'a>,
    column_pixel_height: f64,
    brightness_level: usize,
}

impl<'a> ColumnRenderer<'a> {
    pub fn init(
        screen_x: &'a usize,
        screen_width: &'a usize,
        screen_height_usize: &'a usize,
        screen_height_f64: &f64,
        screen_half_height: &'a usize,
        nearest_wall_intersection: &'a WallCollision,
        camera_position: &Point2D,
    ) -> Self {
        let distance = Point2D::dist(camera_position, &nearest_wall_intersection.intersection);
        let height = if distance != 0.0 {
            WALL_HEIGHT * screen_height_f64 / distance
        } else {
            0.0
        };
        ColumnRenderer {
            screen_x,
            screen_width,
            screen_height: screen_height_usize,
            screen_half_height,
            nearest_wall_intersection,
            column_pixel_height: height,
            brightness_level: distance_to_brightness_level(distance),
        }
    }

    pub fn render_next_pixel(&self, screen_buffer: &mut ScreenBuffer) -> bool {
        let wall_x_pos = self
            .nearest_wall_intersection
            .wall
            .get_wall_space_x_position(&self.nearest_wall_intersection.intersection);
        let wall_painting_indexes = self
            .nearest_wall_intersection
            .wall
            .get_painting_indexes_in_column(wall_x_pos);

        let mut starting_wall_position = 0;
        let mut column_height_usize = self.column_pixel_height as usize;
        if column_height_usize > *self.screen_height {
            starting_wall_position = (column_height_usize - self.screen_height) / 2;
            column_height_usize = *self.screen_height;
        }

        let column_half_height = column_height_usize / 2;
        let column_top_pixel = self.screen_half_height + column_half_height;
        let column_bottom_pixel = self.screen_half_height - column_half_height;

        let bottom_pixel_index = self.screen_x + (column_bottom_pixel * self.screen_width);
        let top_pixel_index = self.screen_x + (column_top_pixel * self.screen_width);

        let mut wall_pixel_index = starting_wall_position;
        let mut pixel_index = bottom_pixel_index;
        while pixel_index < top_pixel_index {
            let wall_y_pos = wall_pixel_index as f64 / self.column_pixel_height;

            let colour = self
                .nearest_wall_intersection
                .wall
                .get_wall_colour_or_painting_colour_at_position(
                    wall_x_pos,
                    wall_y_pos,
                    &wall_painting_indexes,
                );
            let colour = colour.at_brightness(self.brightness_level);
            screen_buffer.render_pixel(pixel_index, colour);

            wall_pixel_index += 1;
            pixel_index += self.screen_width;
        }

        false
    }
}
