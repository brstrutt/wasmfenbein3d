use super::{
    distance_to_brightness_level::distance_to_brightness_level, screen_buffer::ScreenBuffer,
};
use crate::core::{
    primitives::point2d::Point2D,
    world::{wall::WALL_HEIGHT, walls::WallCollision},
};

pub struct ColumnRenderer<'x, 'w> {
    screen_x: &'x usize,
    nearest_wall_intersection: &'w WallCollision<'w>,
    column_pixel_height: f64,
    brightness_level: usize,
}

impl<'x, 'w> ColumnRenderer<'x, 'w> {
    pub fn init(
        screen_x: &'x usize,
        nearest_wall_intersection: &'w WallCollision,
        camera_position: &Point2D,
        screen_height: &f64,
    ) -> Self {
        let distance = Point2D::dist(camera_position, &nearest_wall_intersection.intersection);
        let height = if distance != 0.0 {
            WALL_HEIGHT * screen_height / distance
        } else {
            0.0
        };
        ColumnRenderer {
            screen_x,
            nearest_wall_intersection,
            column_pixel_height: height,
            brightness_level: distance_to_brightness_level(distance),
        }
    }

    pub fn render_next_pixel(&self, screen_buffer: &mut ScreenBuffer) -> bool {
        screen_buffer.render_textured_column(
            self.screen_x,
            self.column_pixel_height,
            self.nearest_wall_intersection,
            self.brightness_level,
        );
        false
    }
}
