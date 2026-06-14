use std::{cell::RefCell, rc::Rc};
use crate::{main_canvas::MainCanvas, render::rgb::RGB, state::GameState, web};

pub mod screen;
pub mod rgb;

pub fn setup(state: Rc<RefCell<GameState>>, canvas: Rc<RefCell<MainCanvas>>) {
    web::window::run_function_every_animation_frame(move || {
        render(&canvas, &state);
    });
}

pub fn render(canvas: &RefCell<MainCanvas>, state: &RefCell<GameState>) {
    let render_start_time = web::window::now_in_ms();

    let canvas = canvas.borrow();
    let mut state = state.borrow_mut();
    screen::clear(&canvas);

    const WALL_COLOUR: RGB = RGB {red: 30, green: 125, blue: 30};

    for x in 0..=canvas.element.width() {
        let ray = state.world.camera.ray_for_column(x, canvas.element.height(), canvas.element.width());
        let wall_distance = state.world.dist_to_wall(&ray);

        if wall_distance.is_some() {
            let distance = wall_distance.unwrap();
            if distance != 0.0 {
                let height = canvas.element.height() as f64 / distance;
                screen::render_column(&canvas, x, height as u32, &(WALL_COLOUR / (distance/5.0).max(1.0)));
            }
        }
    }

    let render_end_time = web::window::now_in_ms();
    state.last_time_to_render_one_frame_ms = render_end_time - render_start_time;
}
