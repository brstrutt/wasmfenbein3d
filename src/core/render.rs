use crate::core::{render::column_data::ColumnData, state::GameState, world::wall::WALL_HEIGHT};
use column_renderer::ColumnRenderer;
use distance_to_brightness_level::distance_to_brightness_level;
use screen_buffer::ScreenBuffer;

use std::{cell::RefCell, rc::Rc};

mod column_data;
mod column_renderer;
mod distance_to_brightness_level;
pub mod rgb;
pub mod rgb_brightness_lookup_table;
pub mod rgb_palette;
pub mod rgbv;
mod row_renderer;
pub mod screen_buffer;
pub mod texel_provider;
pub mod texture;
pub mod tiling_texture;

pub fn render_to_screen_buffer(
    screen_buffer: &Rc<RefCell<ScreenBuffer>>,
    state: &RefCell<GameState>,
) {
    let state = state.borrow_mut();
    let mut screen_buffer = screen_buffer.borrow_mut();

    screen_buffer.reset_draw_history();
    render_walls(&mut screen_buffer, &state);
    render_background(&mut screen_buffer, &state);
}

fn render_background(screen_buffer: &mut ScreenBuffer, state: &GameState) {
    let camera = state.world.camera.clone();
    let half_screen_height = screen_buffer.height as f64 / 2.0;

    let half_wall_height = half_screen_height * WALL_HEIGHT;

    for y in 0..screen_buffer.height {
        let y_relative_to_center = y as f64 - half_screen_height;
        let dist_to_floor = ((1.0 / y_relative_to_center) * half_wall_height).abs();
        let texture = if y_relative_to_center.is_sign_positive() {
            &state.world.floor
        } else {
            &state.world.ceiling
        };

        row_renderer::render_row(
            &y,
            &camera,
            dist_to_floor,
            texture,
            distance_to_brightness_level(dist_to_floor),
            screen_buffer,
        );
    }
}

fn render_walls(screen_buffer: &mut ScreenBuffer, state: &GameState) {
    let screen_height_usize = screen_buffer.height;
    let screen_height_f64 = screen_height_usize as f64;

    for x in 0..screen_buffer.width {
        let ray = state.world.camera.ray_for_column(x);
        let wall_intersection = state.world.nearest_wall_intersecting_ray(&ray);

        if let Some(wall_intersection) = wall_intersection {
            let column_data = ColumnData::init(
                &wall_intersection,
                &state.world.camera.ray.origin,
                &screen_height_f64,
            );
            let mut renderer =
                ColumnRenderer::init(&x, &screen_height_f64, &column_data, screen_buffer);
            renderer.render_column(screen_buffer);
        }
    }
}
