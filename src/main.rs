use std::{cell::RefCell, rc::Rc};

use wasmfenbein3d::core::{
    controls, hud,
    render::{self, screen::ScreenBuffer, textures::Textures},
    state::GameState,
    web::{access, main_canvas},
};

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    log::info!("Starting up!");

    main_canvas::setup();
    main_canvas::update_canvas_size();

    let screen_width = access::main_canvas().width() as usize;
    let screen_height = access::main_canvas().height() as usize;

    let screen_buffer = Rc::new(RefCell::new(ScreenBuffer::setup(
        screen_width,
        screen_height,
    )));

    let state = Rc::new(RefCell::new(GameState::setup(screen_width, screen_height)));
    let textures = Rc::new(RefCell::new(Textures::load()));

    controls::setup(state.clone());
    render::setup(state.clone(), screen_buffer.clone(), textures.clone());
    hud::setup(state.clone());
}
