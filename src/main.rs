use std::{cell::RefCell, rc::Rc};
mod controls;
mod hud;
mod textures;
mod web;
mod world;

use wasmfenbein3d::core::{
    render::{
        render_to_screen_buffer, rgb_palette::RgbPalette,
        screen_buffer_row_first::ScreenBufferRowFirst,
    },
    state::GameState,
};

use crate::{
    web::{access, main_canvas},
    world::load_walls,
};

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    log::info!("Starting up!");

    main_canvas::setup();
    main_canvas::update_canvas_size();

    let screen_width = access::main_canvas().width() as usize;
    let screen_height = access::main_canvas().height() as usize;

    let screen_buffer = Rc::new(RefCell::new(ScreenBufferRowFirst::setup(
        screen_width,
        screen_height,
    )));

    let mut palette = RgbPalette::new();
    let walls = load_walls(&mut palette);
    let floor_texture = textures::big_floor::load_texture(&mut palette);
    let ceiling_texture = textures::floor::load_texture(&mut palette);

    let state = Rc::new(RefCell::new(GameState::setup(
        screen_width,
        screen_height,
        walls,
        &mut palette,
        floor_texture.clone(),
        ceiling_texture,
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
}
