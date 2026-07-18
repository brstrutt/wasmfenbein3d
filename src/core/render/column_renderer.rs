use super::{
    column_data::ColumnData, distance_to_brightness_level::distance_to_brightness_level,
    screen_buffer::ScreenBuffer,
};
use crate::core::world::{painting::Painting, wall::WALL_HEIGHT};

pub struct ColumnRenderer<'a> {
    column: &'a ColumnData<'a>,
    screen: ScreenSpace<'a>,
    wall_space: WallSpace,
    brightness_level: usize,
}

struct ScreenSpace<'a> {
    width: &'a usize,
    column_top_pixel_index: usize,
    current_pixel_index: usize,
}

struct WallSpace {
    y: f64,
    y_increment: f64,
    current_painting: Option<PaintingSpace>,
    next_painting_index: Option<usize>,
}

struct PaintingSpace {
    index: usize,
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
        let next_painting_index = if column.paintings.len() < 1 {
            None
        } else {
            Some(0)
        };

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
                next_painting_index,
            },
            brightness_level: distance_to_brightness_level(column.distance_from_camera),
        }
    }

    pub fn render_next_pixel(&mut self, screen_buffer: &mut ScreenBuffer) -> bool {
        let wall = &self.column.nearest_wall_intersection.wall;
        let colour = if let Some(current_painting) = &self.wall_space.current_painting {
            let painting = &self.column.paintings[current_painting.index];
            let wall_space_y = self.wall_space.y - painting.top_left_corner.y;
            painting.texture.get_texel(
                (current_painting.x * painting.texture.width_f64() / painting.width) as isize,
                (wall_space_y * painting.texture.height_f64() / painting.height) as isize,
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
            &self.column.paintings,
            &mut self.wall_space.current_painting,
            &mut self.wall_space.next_painting_index,
        );
        Self::try_leave_current_painting(
            self.wall_space.y,
            &self.column.paintings,
            &mut self.wall_space.current_painting,
        );

        self.screen.current_pixel_index < self.screen.column_top_pixel_index
    }

    fn try_enter_next_painting(
        wall_x: f64,
        wall_y: f64,
        paintings_in_column: &[&Painting],
        current_painting: &mut Option<PaintingSpace>,
        mut next_painting_index: &mut Option<usize>,
    ) {
        if let Some(next_index) = &mut next_painting_index
            && wall_y >= paintings_in_column[*next_index].top_left_corner.y
        {
            *current_painting = Some(PaintingSpace {
                index: *next_index,
                x: wall_x - paintings_in_column[*next_index].top_left_corner.x,
            });
            *next_index += 1;
            if *next_index >= paintings_in_column.len() {
                *next_painting_index = None;
            }
        }
    }

    fn try_leave_current_painting(
        wall_y: f64,
        paintings_in_column: &[&Painting],
        current_painting: &mut Option<PaintingSpace>,
    ) {
        if let Some(current) = &current_painting
            && wall_y >= paintings_in_column[current.index].bottom_right_corner.y
        {
            *current_painting = None;
        }
    }
}
