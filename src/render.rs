use crate::{
    primitives::point2d::Point2D, render::{screen::ScreenBuffer, texture::Texture}, state::GameState, web::{self, main_canvas},
};
use std::{cell::RefCell, rc::Rc};

pub mod rgb;
pub mod screen;
pub mod texture;

pub fn setup(
    state: Rc<RefCell<GameState>>,
    screen_buffer: Rc<RefCell<ScreenBuffer>>,
) {
    web::window::run_function_every_animation_frame(move || {
        render(&screen_buffer, &state);
    });
}

pub fn render(
    screen_buffer: &Rc<RefCell<ScreenBuffer>>,
    state: &RefCell<GameState>,
) {
    let render_start_time = web::window::now_in_ms();

    let mut state = state.borrow_mut();
    let mut screen_buffer = screen_buffer.borrow_mut();
    let screen_height = screen_buffer.height as f64;

    let wall_texture = Texture::get_default();

    for x in 0..screen_buffer.width {
        let ray = state.world.camera.ray_for_column(x, screen_buffer.height, screen_buffer.width);
        let wall_intersection = state.world.nearest_wall_intersection(&ray);

        let mut height = 0.0;

        if let Some(wall_intersection) = wall_intersection {
            let distance = Point2D::dist(&state.world.camera.origin, &wall_intersection.intersection);

            if distance != 0.0 {
                height = 2.0 * screen_height / distance;
            }
        }

        screen_buffer.render_textured_column(
            &x,
            height,
            &wall_texture,
        );
    }

    main_canvas::render_screen_buffer(&screen_buffer);

    let render_end_time = web::window::now_in_ms();
    state.last_time_to_render_one_frame_ms = render_end_time - render_start_time;
}
