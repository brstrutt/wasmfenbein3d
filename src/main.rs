use std::{cell::RefCell, rc::Rc};
mod controls;
mod hud;
mod web;

use wasmfenbein3d::core::{
    render::{
        render_to_screen_buffer, rgb_palette::RgbPalette, screen::ScreenBuffer, textures::Textures,
    },
    state::GameState,
};

use crate::web::{access, main_canvas};

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

    let mut palette = RgbPalette::new();
    let textures = Rc::new(RefCell::new(Textures::load(&mut palette)));
    let state = Rc::new(RefCell::new(GameState::setup(
        screen_width,
        screen_height,
        &textures,
        &mut palette,
    )));

    controls::setup(state.clone());
    hud::setup(state.clone());
    web::window::run_function_every_animation_frame(move || {
        let render_start_time = web::window::now_in_ms();
        render_to_screen_buffer(&screen_buffer, &state, &textures);
        main_canvas::render_screen_buffer(&screen_buffer.borrow());
        let render_end_time = web::window::now_in_ms();

        let mut state = state.borrow_mut();
        state.last_time_to_render_one_frame_ms = render_end_time - render_start_time;
    });
}
