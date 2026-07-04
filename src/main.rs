mod controls;
mod hud;
mod motion;
mod primitives;
mod render;
mod state;
mod utils;
mod web;
mod world;

use std::{cell::RefCell, rc::Rc};

use crate::{
    render::screen::ScreenBuffer,
    state::GameState,
    web::{access, main_canvas},
};

fn main() {
    console_error_panic_hook::set_once();

    web::log::log("Starting up!");

    main_canvas::setup();
    main_canvas::update_canvas_size();

    let screen_width = access::main_canvas().width() as usize;
    let screen_height = access::main_canvas().height() as usize;

    let screen_buffer = Rc::new(RefCell::new(ScreenBuffer::setup(
        screen_width,
        screen_height,
    )));

    let state = Rc::new(RefCell::new(GameState::setup(screen_width, screen_height)));

    controls::setup(state.clone());
    render::setup(state.clone(), screen_buffer.clone());
    hud::setup(state.clone());
}
