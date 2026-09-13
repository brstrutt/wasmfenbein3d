use std::{cell::RefCell, rc::Rc};
mod controls;
mod hud;
mod textures;
mod web;

use wasmfenbein3d::{
    render::render_to_screen_buffer,
    state::{State, textures::TextureLibrary, world},
};

use crate::web::main_canvas;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    let mut timing_logger = TimingLogger::new();
    log::info!("Starting up!");

    let state = Rc::new(RefCell::new(State::setup(
        main_canvas::setup_screen_buffer(),
        world::World::load(
            &TextureLibrary::load(&textures::TILING_TEXTURES, &textures::TEXTURES),
            include_str!("./world/data.json"),
        ),
    )));

    timing_logger.log_time("State Setup!!");

    controls::setup(state.clone());
    hud::setup(state.clone());
    web::window::run_function_every_animation_frame(move || {
        let render_start_time = web::window::now_in_ms();
        render_to_screen_buffer(&state);
        let mut state = state.borrow_mut();
        main_canvas::render_screen_buffer(&state.screen_buffer);
        let render_end_time = web::window::now_in_ms();

        state.stats.render_frame.last_duration_ms = render_end_time - render_start_time;
        state.stats.render_frame.last_time_ms = render_start_time;
    });

    timing_logger.log_time("Setup complete!");
}

struct TimingLogger {
    last_call_time_ms: f64,
}

impl TimingLogger {
    pub fn new() -> TimingLogger {
        TimingLogger {
            last_call_time_ms: web::window::now_in_ms(),
        }
    }

    pub fn log_time(&mut self, msg: &str) {
        let current_time = web::window::now_in_ms();
        log::info!(
            "Time taken: {}ms, Msg: {}",
            current_time - self.last_call_time_ms,
            msg
        );
        self.last_call_time_ms = current_time;
    }
}
