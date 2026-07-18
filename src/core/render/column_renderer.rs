use super::{
    column_data::ColumnData, distance_to_brightness_level::distance_to_brightness_level,
    screen_buffer::ScreenBuffer, texel_provider::TexelProvider,
};
use crate::core::world::{painting::Painting, wall::WALL_HEIGHT};

pub struct ColumnRenderer<'a> {
    column: &'a ColumnData<'a>,
    screen: ScreenSpace<'a>,
    wall_space: WallSpace,
    brightness_level: usize,
    render_plan: Vec<ColumnSegment<'a>>,
}

struct ScreenSpace<'a> {
    current_pixel_index: usize,
    column_last_pixel_index: usize,
    pixel_increment: &'a usize,
}

struct WallSpace {
    x: f64,
    y: f64,
    y_increment: f64,
}

struct ColumnSegment<'a> {
    texture: &'a dyn TexelProvider,
    wall_space_start_y: f64,
    wall_space_end_y: f64,
    texture_space_x: f64,
    texture_space_start_y: f64,
    texture_space_end_y: f64,
    texture_space_y_increment: f64,
}

impl<'a> ColumnRenderer<'a> {
    pub fn init(
        screen_x: &'a usize,
        screen_width: &'a usize,
        screen_height: &'a f64,
        column: &'a ColumnData<'a>,
    ) -> Self {
        let wall_space_pixel_height = WALL_HEIGHT / column.height_pixels;

        let mut wall_start_y = 0.0;
        let mut wall_end_y = WALL_HEIGHT;

        let mut screen_start_y = 0.0;
        let mut screen_end_y = screen_height.clone();

        if column.height_pixels > *screen_height {
            let offscreen_pixel_count = column.height_pixels - screen_height;
            let half_offscreen_pixel_count = offscreen_pixel_count / 2.0;
            let wallspace_adjustment = half_offscreen_pixel_count * wall_space_pixel_height;
            wall_start_y += wallspace_adjustment;
            wall_end_y -= wallspace_adjustment;
        } else {
            let pixels_from_edge = (screen_height - column.height_pixels) / 2.0;
            screen_start_y += pixels_from_edge;
            screen_end_y -= pixels_from_edge;
        }

        let mut segment_start_y = wall_start_y;
        let mut render_plan = vec![];
        for painting in &column.paintings {
            if painting.top_left_corner.y > segment_start_y {
                let texture = column.nearest_wall_intersection.wall.texture.as_ref();
                render_plan.push(ColumnSegment {
                    texture: texture,
                    wall_space_start_y: segment_start_y,
                    wall_space_end_y: painting.top_left_corner.y,
                    texture_space_x: column.wall_x_pos * texture.width_f64(),
                    texture_space_start_y: segment_start_y * texture.height_f64(),
                    texture_space_end_y: painting.top_left_corner.y * texture.height_f64()
                        / WALL_HEIGHT,
                    texture_space_y_increment: wall_space_pixel_height * texture.height_f64(),
                });
                segment_start_y = painting.top_left_corner.y;
            }
            if painting.bottom_right_corner.y > segment_start_y {
                let wall_space_end_y = painting.bottom_right_corner.y.min(wall_end_y);
                render_plan.push(ColumnSegment {
                    texture: painting.texture.as_ref(),
                    wall_space_start_y: segment_start_y,
                    wall_space_end_y,
                    texture_space_x: (column.wall_x_pos - painting.top_left_corner.x)
                        * painting.texture.width_f64()
                        / painting.width,
                    texture_space_start_y: (segment_start_y - painting.top_left_corner.y)
                        * painting.texture.height_f64()
                        / painting.height,
                    texture_space_end_y: (wall_space_end_y - painting.top_left_corner.y)
                        * painting.texture.height_f64()
                        / painting.height,
                    texture_space_y_increment: wall_space_pixel_height
                        * painting.texture.height_f64()
                        / painting.height,
                });
                segment_start_y = wall_space_end_y;
            }
        }

        if segment_start_y < wall_end_y {
            let texture = column.nearest_wall_intersection.wall.texture.as_ref();
            render_plan.push(ColumnSegment {
                texture: column.nearest_wall_intersection.wall.texture.as_ref(),
                wall_space_start_y: segment_start_y,
                wall_space_end_y: wall_end_y,
                texture_space_x: column.wall_x_pos * texture.width_f64(),
                texture_space_start_y: segment_start_y * texture.height_f64(),
                texture_space_end_y: wall_end_y * texture.height_f64() / WALL_HEIGHT,
                texture_space_y_increment: wall_space_pixel_height * texture.height_f64(),
            });
        }

        ColumnRenderer {
            column,
            screen: ScreenSpace {
                current_pixel_index: screen_x + (screen_start_y as usize * screen_width),
                column_last_pixel_index: screen_x + (screen_end_y as usize * screen_width),
                pixel_increment: screen_width,
            },
            wall_space: WallSpace {
                x: column.wall_x_pos,
                y: wall_start_y,
                y_increment: wall_space_pixel_height,
            },
            brightness_level: distance_to_brightness_level(column.distance_from_camera),
            render_plan,
        }
    }

    pub fn render_column(&mut self, screen_buffer: &mut ScreenBuffer) {
        for segment in &self.render_plan {
            let mut tex_space_y = segment.texture_space_start_y;
            while self.wall_space.y < segment.wall_space_end_y
                && self.screen.current_pixel_index < self.screen.column_last_pixel_index
            {
                let colour = segment
                    .texture
                    .get_texel(segment.texture_space_x as isize, tex_space_y as isize)
                    .at_brightness(self.brightness_level);

                screen_buffer.render_pixel(self.screen.current_pixel_index, colour);

                self.screen.current_pixel_index += self.screen.pixel_increment;
                self.wall_space.y += self.wall_space.y_increment;
                tex_space_y += segment.texture_space_y_increment;
            }
        }
    }
}
