use std::{cell::RefCell, rc::Rc};
mod controls;
mod hud;
mod textures;
mod web;

use wasmfenbein3d::state::{State, textures::TextureLibrary, world};

use crate::web::{access, main_canvas};

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

    wasmfenbein3d::wasmrender::setup(state.clone(), access::main_canvas());
    wasmfenbein3d::wasmcontrols::setup(state.clone(), access::main_canvas(), |id: &str| {
        controls::on_click(id);
    });

    hud::setup(state);

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
