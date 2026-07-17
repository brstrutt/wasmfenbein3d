use super::{
    distance_to_brightness_level::distance_to_brightness_level, screen_buffer::ScreenBuffer,
};
use crate::core::{
    primitives::point2d::Point2D,
    world::{wall::WALL_HEIGHT, walls::WallCollision},
};

pub struct ColumnRenderer<'a> {
    screen: ScreenSpace<'a>,
    wall_space: WallSpace,
    nearest_wall_intersection: &'a WallCollision<'a>,
    brightness_level: usize,
}

struct ScreenSpace<'a> {
    width: &'a usize,
    column_height_f64: f64,
    column_top_pixel_index: usize,
    current_pixel_index: usize,
}

struct WallSpace {
    x: f64,
    painting_in_column_ids: Vec<usize>,
    current_pixel_index: f64,
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

        let wall_x_pos = nearest_wall_intersection
            .wall
            .get_wall_space_x_position(&nearest_wall_intersection.intersection);

        let mut column_height_usize = height as usize;
        let mut wall_starting_position = 0;
        if column_height_usize > *screen_height_usize {
            wall_starting_position = (column_height_usize - screen_height_usize) / 2;
            column_height_usize = *screen_height_usize;
        }

        let column_half_height = column_height_usize / 2;
        let column_top_pixel = screen_half_height + column_half_height;
        let column_bottom_pixel = screen_half_height - column_half_height;

        let wall_painting_indexes = nearest_wall_intersection
            .wall
            .get_painting_indexes_in_column(wall_x_pos);

        ColumnRenderer {
            screen: ScreenSpace {
                width: screen_width,
                column_height_f64: height,
                column_top_pixel_index: screen_x + (column_top_pixel * screen_width),
                current_pixel_index: screen_x + (column_bottom_pixel * screen_width),
            },
            wall_space: WallSpace {
                x: wall_x_pos,
                painting_in_column_ids: wall_painting_indexes,
                current_pixel_index: wall_starting_position as f64,
            },
            nearest_wall_intersection,
            brightness_level: distance_to_brightness_level(distance),
        }
    }

    pub fn render_next_pixel(&mut self, screen_buffer: &mut ScreenBuffer) -> bool {
        let wall_y_pos = self.wall_space.current_pixel_index / self.screen.column_height_f64;

        let colour = self
            .nearest_wall_intersection
            .wall
            .get_wall_colour_or_painting_colour_at_position(
                self.wall_space.x,
                wall_y_pos,
                &self.wall_space.painting_in_column_ids,
            );
        let colour = colour.at_brightness(self.brightness_level);
        screen_buffer.render_pixel(self.screen.current_pixel_index, colour);

        self.wall_space.current_pixel_index += 1.0;
        self.screen.current_pixel_index += self.screen.width;

        self.screen.current_pixel_index < self.screen.column_top_pixel_index
    }
}
