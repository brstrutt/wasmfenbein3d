use crate::core::{
    primitives::point2d::Point2D,
    render::{rgb_brightness_lookup_table::BRIGHTNESS_STEPS_F64, screen_buffer::ScreenBuffer},
    state::GameState,
    world::wall::WALL_HEIGHT,
};
use std::{cell::RefCell, rc::Rc};

pub mod rgb;
pub mod rgb_brightness_lookup_table;
pub mod rgb_palette;
pub mod rgbv;
pub mod screen_buffer;
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

        screen_buffer.render_textured_row(
            &y,
            &camera,
            dist_to_floor,
            texture,
            get_light_falloff(dist_to_floor),
        );
    }
}

fn render_walls(screen_buffer: &mut ScreenBuffer, state: &GameState) {
    let screen_height = screen_buffer.height as f64;

    for x in 0..screen_buffer.width {
        let ray = state.world.camera.ray_for_column(x);
        let wall_intersection = state.world.nearest_wall_intersecting_ray(&ray);

        let mut height = 0.0;
        if let Some(wall_intersection) = wall_intersection {
            let distance = Point2D::dist(
                &state.world.camera.ray.origin,
                &wall_intersection.intersection,
            );

            if distance != 0.0 {
                height = WALL_HEIGHT * screen_height / distance;
            }

            screen_buffer.render_textured_column(
                &x,
                height,
                &wall_intersection,
                get_light_falloff(distance),
            );
        }
    }
}

fn get_light_falloff(distance: f64) -> usize {
    (BRIGHTNESS_STEPS_F64 - (distance * 8.0)) as usize
}
