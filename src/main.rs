use std::{cell::RefCell, rc::Rc};
mod controls;
mod hud;
mod textures;
mod web;

use wasmfenbein3d::core::{
    render::{render_to_screen_buffer, screen_buffer_column_first::ScreenBufferColumnFirst},
    state::{GameState, textures::TextureLibrary, world},
};

use crate::web::{access, main_canvas};

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    let mut timing_logger = TimingLogger::new();
    log::info!("Starting up!");

    main_canvas::update_canvas_size();

    let screen_width = access::main_canvas().height() as usize;
    let screen_height = access::main_canvas().width() as usize;

    let screen_buffer = Rc::new(RefCell::new(ScreenBufferColumnFirst::setup(
        screen_width,
        screen_height,
    )));

    timing_logger.log_time("Canvas setup!");

    let textures = TextureLibrary::load(&textures::TILING_TEXTURES, &textures::TEXTURES);
    let world = world::World::load(&textures, include_str!("./world/data.json"));

    timing_logger.log_time("World loaded!!");

    let state = Rc::new(RefCell::new(GameState::setup(
        screen_width,
        screen_height,
        world,
    )));

    controls::setup(state.clone());
    hud::setup(state.clone(), screen_buffer.clone());
    web::window::run_function_every_animation_frame(move || {
        let render_start_time = web::window::now_in_ms();
        render_to_screen_buffer(&screen_buffer, &state);
        main_canvas::render_screen_buffer(screen_buffer.borrow());
        let render_end_time = web::window::now_in_ms();

        let mut state = state.borrow_mut();
        state.last_time_to_render_one_frame_ms = render_end_time - render_start_time;
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
