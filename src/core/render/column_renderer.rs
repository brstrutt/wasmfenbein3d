use std::{iter::Peekable, slice::Iter};

use super::{
    column_data::ColumnData, distance_to_brightness_level::distance_to_brightness_level,
    screen_buffer::ScreenBuffer, texel_provider::TexelProvider,
};
use crate::core::world::{painting::Painting, wall::WALL_HEIGHT};

pub struct ColumnRenderer<'a> {
    column: &'a ColumnData<'a>,
    screen: ScreenSpace<'a>,
    wall_space: WallSpace<'a>,
    brightness_level: usize,
}

struct ScreenSpace<'a> {
    width: &'a usize,
    column_top_pixel_index: usize,
    current_pixel_index: usize,
}

struct WallSpace<'a> {
    y: f64,
    y_increment: f64,
    current_painting: Option<PaintingSpace<'a>>,
    next_painting: Peekable<Iter<'a, &'a Painting>>,
}

struct PaintingSpace<'a> {
    painting: &'a Painting,
    x: f64,
}

impl<'a> ColumnRenderer<'a> {
    pub fn init(
        screen_x: &'a usize,
        screen_width: &'a usize,
        screen_height_usize: &'a usize,
        screen_half_height: &'a usize,
        column: &'a ColumnData<'a>,
    ) -> Self {
        let mut column_height_usize = column.height_pixels as usize;
        let mut wall_starting_position = 0;
        if column_height_usize > *screen_height_usize {
            wall_starting_position = (column_height_usize - screen_height_usize) / 2;
            column_height_usize = *screen_height_usize;
        }

        let column_half_height = column_height_usize / 2;
        let column_top_pixel = screen_half_height + column_half_height;
        let column_bottom_pixel = screen_half_height - column_half_height;

        let wall_y = wall_starting_position as f64 / column.height_pixels;

        ColumnRenderer {
            column,
            screen: ScreenSpace {
                width: screen_width,
                column_top_pixel_index: screen_x + (column_top_pixel * screen_width),
                current_pixel_index: screen_x + (column_bottom_pixel * screen_width),
            },
            wall_space: WallSpace {
                y: wall_y,
                y_increment: 1.0 / column.height_pixels,
                current_painting: None,
                next_painting: column.paintings.iter().peekable(),
            },
            brightness_level: distance_to_brightness_level(column.distance_from_camera),
        }
    }

    pub fn render_column(&mut self, screen_buffer: &mut ScreenBuffer) {
        while self.screen.current_pixel_index < self.screen.column_top_pixel_index {
            let next_y = self.wall_space.y + self.wall_space.y_increment;
            if let Some(current_painting) = &self.wall_space.current_painting {
                let wall_space_y = self.wall_space.y - current_painting.painting.top_left_corner.y;
                let colour = current_painting
                    .painting
                    .texture
                    .get_texel(
                        (current_painting.x * current_painting.painting.texture.width_f64()
                            / current_painting.painting.width) as isize,
                        (wall_space_y * current_painting.painting.texture.height_f64()
                            / current_painting.painting.height) as isize,
                    )
                    .at_brightness(self.brightness_level);

                screen_buffer.render_pixel(self.screen.current_pixel_index, colour);

                if next_y >= current_painting.painting.bottom_right_corner.y {
                    self.wall_space.current_painting = None;
                }
            } else {
                let colour = self
                    .column
                    .nearest_wall_intersection
                    .wall
                    .texture
                    .get_texel(
                        (self.column.wall_x_pos
                            * self
                                .column
                                .nearest_wall_intersection
                                .wall
                                .texture
                                .width_f64()) as isize,
                        (self.wall_space.y
                            * self
                                .column
                                .nearest_wall_intersection
                                .wall
                                .texture
                                .height_f64()
                            * WALL_HEIGHT) as isize,
                    )
                    .at_brightness(self.brightness_level);

                screen_buffer.render_pixel(self.screen.current_pixel_index, colour);

                if let Some(next_painting_data) = &self.wall_space.next_painting.peek()
                    && next_y >= next_painting_data.top_left_corner.y
                {
                    self.wall_space.current_painting = Some(PaintingSpace {
                        painting: next_painting_data,
                        x: self.column.wall_x_pos - next_painting_data.top_left_corner.x,
                    });
                    self.wall_space.next_painting.next();
                }
            };

            self.wall_space.y = next_y;
            self.screen.current_pixel_index += self.screen.width;
        }
    }
}
