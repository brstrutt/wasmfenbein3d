use std::{cell::RefCell, rc::Rc};
mod controls;
mod hud;
mod textures;
mod web;

use wasmfenbein3d::core::{
    render::{render_to_screen_buffer, screen_buffer_column_first::ScreenBufferColumnFirst},
    state::{GameState, world::load_from_json::load_walls_from_json},
};

use crate::web::{access, main_canvas};

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    let start_time = web::window::now_in_ms();
    log::info!("Starting up!");

    main_canvas::update_canvas_size();

    let screen_width = access::main_canvas().height() as usize;
    let screen_height = access::main_canvas().width() as usize;

    let screen_buffer = Rc::new(RefCell::new(ScreenBufferColumnFirst::setup(
        screen_width,
        screen_height,
    )));

    let canvas_finish_time = web::window::now_in_ms();
    log::info!(
        "Canvas setup! Time taken: {}ms",
        canvas_finish_time - start_time
    );

    let textures = textures::load();
    let walls = load_walls_from_json(&textures, include_str!("./world/data.json"));

    let world_load_finish_time = web::window::now_in_ms();
    log::info!(
        "World loaded! Time taken: {}ms",
        world_load_finish_time - canvas_finish_time
    );

    let state = Rc::new(RefCell::new(GameState::setup(
        screen_width,
        screen_height,
        walls,
        textures.get(textures::BIG_FLOOR.id),
        textures.get(textures::FLOOR.id),
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
    log::info!(
        "Setup complete! Total time taken: {}ms",
        web::window::now_in_ms() - start_time
    );
}
