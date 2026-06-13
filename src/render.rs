use std::{cell::RefCell, rc::Rc};
use crate::{main_canvas::MainCanvas, render::rgb::RGB, state::GameState, web};

pub mod screen;
pub mod rgb;

pub fn setup(state: Rc<RefCell<GameState>>, canvas: MainCanvas) {
    web::window::run_function_every_animation_frame(move || {
        render(&canvas, &state);
    });
}

pub fn render(canvas: &MainCanvas, state: &RefCell<GameState>) {
    screen::clear(canvas);
    let state = state.borrow();

    for x in 0..=canvas.element.width() {
        let ray = state.world.camera.ray_for_column(x, canvas.element.height(), canvas.element.width());
        let wall_distance = state.world.dist_to_wall(&ray);

        if wall_distance.is_some() {
            let distance = wall_distance.unwrap();
            if distance != 0.0 {
                let height = canvas.element.height() as f64 / distance;
                screen::render_column(canvas, x, height as u32, &RGB {red: 30, green: 125, blue: 30});
            }
        }
    }
}
