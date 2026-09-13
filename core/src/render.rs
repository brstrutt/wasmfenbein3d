use crate::{render::column_data::ColumnData, state::State, state::world::wall::WALL_HEIGHT};
use column_renderer::ColumnRenderer;
use distance_to_brightness_level::distance_to_brightness_level;

use std::cell::RefCell;

pub mod camera;
mod colour;
mod column_data;
mod column_renderer;
mod distance_to_brightness_level;
pub mod rgb;
pub mod rgb_brightness_lookup_table;
pub mod rgb_palette;
pub mod rgbv;
mod row_renderer;
pub mod screen_buffer;
pub mod screen_buffer_column_first;
pub mod screen_buffer_row_first;
pub mod texel_provider;
pub mod texture;
pub mod tiling_texture;

pub fn render_to_screen_buffer(state: &RefCell<State>) {
    let mut state = state.borrow_mut();
    state.screen_buffer.reset_draw_history();
    render_walls(&mut state);
    render_background(&mut state);
}

fn render_background(state: &mut State) {
    let screen_buffer = &state.screen_buffer;
    let camera = state.camera.clone();
    let half_screen_height = screen_buffer.height() as f64 / 2.0;

    let half_wall_height = half_screen_height * WALL_HEIGHT;

    for y in 0..screen_buffer.height() {
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
            texture.as_ref().as_ref(),
            distance_to_brightness_level(dist_to_floor),
            &mut state.screen_buffer,
        );
    }
}

fn render_walls(state: &mut State) {
    let screen_buffer = &state.screen_buffer;
    let screen_height_f64 = screen_buffer.height() as f64;

    for x in 0..screen_buffer.width() {
        let ray = state.camera.ray_for_column(x);
        let wall_intersection = state.world.nearest_wall_intersecting_ray(&ray);

        if let Some(wall_intersection) = wall_intersection {
            let column_data = ColumnData::init(
                &wall_intersection,
                &state.camera.ray.origin,
                &screen_height_f64,
            );
            let mut renderer =
                ColumnRenderer::init(&x, &screen_height_f64, &column_data, &state.screen_buffer);
            renderer.render_column(&mut state.screen_buffer);
        }
    }
}
