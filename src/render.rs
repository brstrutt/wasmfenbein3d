use crate::{
    web::main_canvas,
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

    const WALL_COLOUR: RGB = RGB {
        red: 30,
        green: 125,
        blue: 30,
    };

    for x in 0..screen_buffer.width {
        let ray = state.world.camera.ray_for_column(x, screen_buffer.height, screen_buffer.width);
        let wall_distance = state.world.dist_to_wall(&ray);

        let mut height = 0;
        let mut wall_color_adjustment = 1.0;

        if wall_distance.is_some() {
            let distance = wall_distance.unwrap();
            wall_color_adjustment = (distance / 5.0).max(1.0);

            if distance != 0.0 {
                height = (2.0 * screen_buffer.height as f64 / distance) as usize;
            }
        }

        screen_buffer.render_column(
            &x,
            height,
            &(WALL_COLOUR / wall_color_adjustment),
        );
    }

    main_canvas::render_screen_buffer(&screen_buffer);

    let render_end_time = web::window::now_in_ms();
    state.last_time_to_render_one_frame_ms = render_end_time - render_start_time;
}
