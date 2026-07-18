use super::{
    distance_to_brightness_level::distance_to_brightness_level, screen_buffer::ScreenBuffer,
};
use crate::core::{
    primitives::point2d::Point2D,
    world::{painting::Painting, wall::WALL_HEIGHT, walls::WallCollision},
};

pub struct ColumnRenderer<'a> {
    screen: ScreenSpace<'a>,
    wall_space: WallSpace<'a>,
    nearest_wall_intersection: &'a WallCollision<'a>,
    brightness_level: usize,
}

struct ScreenSpace<'a> {
    width: &'a usize,
    column_top_pixel_index: usize,
    current_pixel_index: usize,
}

struct WallSpace<'a> {
    x: f64,
    y: f64,
    y_increment: f64,
    paintings_in_column: Vec<&'a Painting>,
    current_painting_index: Option<usize>,
    next_painting_index: Option<usize>,
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

        let wall_y = wall_starting_position as f64 / height;
        let paintings_in_column = nearest_wall_intersection
            .wall
            .get_paintings_in_column(wall_x_pos);
        let next_painting_index = if paintings_in_column.len() < 1 {
            None
        } else {
            Some(0)
        };

        ColumnRenderer {
            screen: ScreenSpace {
                width: screen_width,
                column_top_pixel_index: screen_x + (column_top_pixel * screen_width),
                current_pixel_index: screen_x + (column_bottom_pixel * screen_width),
            },
            wall_space: WallSpace {
                x: wall_x_pos,
                y: wall_y,
                y_increment: 1.0 / height,
                paintings_in_column,
                current_painting_index: None,
                next_painting_index,
            },
            nearest_wall_intersection,
            brightness_level: distance_to_brightness_level(distance),
        }
    }

    pub fn render_next_pixel(&mut self, screen_buffer: &mut ScreenBuffer) -> bool {
        let colour = if let Some(current_painting_index) = self.wall_space.current_painting_index {
            self.nearest_wall_intersection.wall.get_painting_texel(
                self.wall_space.x,
                self.wall_space.y,
                &self.wall_space.paintings_in_column[current_painting_index],
            )
        } else {
            self.nearest_wall_intersection
                .wall
                .get_wall_texel(self.wall_space.x, self.wall_space.y)
        };
        let colour = colour.at_brightness(self.brightness_level);
        screen_buffer.render_pixel(self.screen.current_pixel_index, colour);

        self.wall_space.y += self.wall_space.y_increment;
        self.screen.current_pixel_index += self.screen.width;

        Self::try_enter_next_painting(
            self.wall_space.y,
            &self.wall_space.paintings_in_column,
            &mut self.wall_space.current_painting_index,
            &mut self.wall_space.next_painting_index,
        );
        Self::try_leave_current_painting(
            self.wall_space.y,
            &self.wall_space.paintings_in_column,
            &mut self.wall_space.current_painting_index,
        );

        self.screen.current_pixel_index < self.screen.column_top_pixel_index
    }

    fn try_enter_next_painting(
        wall_y: f64,
        paintings_in_column: &[&Painting],
        current_painting_index: &mut Option<usize>,
        next_painting_index: &mut Option<usize>,
    ) {
        if let Some(next_index) = next_painting_index
            && wall_y >= paintings_in_column[*next_index].top_left_corner.y
        {
            *current_painting_index = Some(*next_index);
            *next_index += 1;
            if *next_index >= paintings_in_column.len() {
                *next_painting_index = None;
            }
        }
    }

    fn try_leave_current_painting(
        wall_y: f64,
        paintings_in_column: &[&Painting],
        current_painting_index: &mut Option<usize>,
    ) {
        if let Some(current_index) = current_painting_index
            && wall_y >= paintings_in_column[*current_index].bottom_right_corner.y
        {
            *current_painting_index = None;
        }
    }
}
