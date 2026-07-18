use std::{iter::Peekable, slice::Iter};

use super::{
    column_data::ColumnData, distance_to_brightness_level::distance_to_brightness_level,
    screen_buffer::ScreenBuffer,
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

    pub fn render_next_pixel(&mut self, screen_buffer: &mut ScreenBuffer) -> bool {
        let wall = &self.column.nearest_wall_intersection.wall;
        let colour = if let Some(current_painting) = &self.wall_space.current_painting {
            let wall_space_y = self.wall_space.y - current_painting.painting.top_left_corner.y;
            current_painting.painting.texture.get_texel(
                (current_painting.x * current_painting.painting.texture.width_f64()
                    / current_painting.painting.width) as isize,
                (wall_space_y * current_painting.painting.texture.height_f64()
                    / current_painting.painting.height) as isize,
            )
        } else {
            wall.texture.get_texel(
                (self.column.wall_x_pos * wall.texture.width_f64()) as isize,
                (self.wall_space.y * wall.texture.height_f64() * WALL_HEIGHT) as isize,
            )
        };
        let colour = colour.at_brightness(self.brightness_level);
        screen_buffer.render_pixel(self.screen.current_pixel_index, colour);

        self.wall_space.y += self.wall_space.y_increment;
        self.screen.current_pixel_index += self.screen.width;

        Self::try_enter_next_painting(
            self.column.wall_x_pos,
            self.wall_space.y,
            &mut self.wall_space.current_painting,
            &mut self.wall_space.next_painting,
        );
        Self::try_leave_current_painting(self.wall_space.y, &mut self.wall_space.current_painting);

        self.screen.current_pixel_index < self.screen.column_top_pixel_index
    }

    fn try_enter_next_painting<'t>(
        wall_x: f64,
        wall_y: f64,
        current_painting: &mut Option<PaintingSpace<'t>>,
        next_painting_iter: &mut Peekable<Iter<&'t Painting>>,
    ) {
        if let Some(next_painting) = &next_painting_iter.peek()
            && wall_y >= next_painting.top_left_corner.y
        {
            *current_painting = Some(PaintingSpace {
                painting: next_painting,
                x: wall_x - next_painting.top_left_corner.x,
            });
            next_painting_iter.next();
        }
    }

    fn try_leave_current_painting(wall_y: f64, current_painting: &mut Option<PaintingSpace>) {
        if let Some(current) = &current_painting
            && wall_y >= current.painting.bottom_right_corner.y
        {
            *current_painting = None;
        }
    }
}
