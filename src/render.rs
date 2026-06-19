use crate::{
    main_canvas::MainCanvas,
    render::{rgb::RGB, screen::ScreenBuffer},
    state::GameState,
    web,
};
use std::{cell::RefCell, rc::Rc};

pub mod rgb;
pub mod screen;

pub fn setup(
    state: Rc<RefCell<GameState>>,
    screen_buffer: Rc<RefCell<ScreenBuffer>>,
    canvas: Rc<RefCell<MainCanvas>>,
) {
    web::window::run_function_every_animation_frame(move || {
        render(&canvas, &screen_buffer, &state);
    });
}

pub fn render(
    canvas: &RefCell<MainCanvas>,
    screen_buffer: &Rc<RefCell<ScreenBuffer>>,
    state: &RefCell<GameState>,
) {
    let render_start_time = web::window::now_in_ms();

    let mut canvas = canvas.borrow_mut();
    let mut state = state.borrow_mut();
    let mut screen_buffer = screen_buffer.borrow_mut();

    const WALL_COLOUR: RGB = RGB {
        red: 30,
        green: 125,
        blue: 30,
    };

    let width = canvas.element.width() as usize;
    let height = canvas.element.height() as usize;
    for x in 0..width {
        let ray = state.world.camera.ray_for_column(x, height, width);
        let wall_distance = state.world.dist_to_wall(&ray);

        if wall_distance.is_some() {
            let distance = wall_distance.unwrap();
            if distance != 0.0 {
                let height = (2.0 * canvas.element.height() as f64 / distance) as usize;
                screen_buffer.render_column(
                    &x,
                    height,
                    &(WALL_COLOUR / (distance / 5.0).max(1.0)),
                );
            }
        }
    }

    canvas.render_screen_buffer(&screen_buffer);

    let render_end_time = web::window::now_in_ms();
    state.last_time_to_render_one_frame_ms = render_end_time - render_start_time;
}
