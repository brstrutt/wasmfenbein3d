mod primitives;
mod render;
mod world;
mod main_canvas;
mod controls;
mod web;
mod state;

use std::{cell::RefCell, rc::Rc};

use crate::{main_canvas::MainCanvas, state::GameState};

fn main() {
    console_error_panic_hook::set_once();

    web::log::log("Starting up!");

    let mut canvas = MainCanvas::init();
    canvas.update_canvas_size();

    let state = Rc::new(RefCell::new(GameState::setup()));

    controls::setup(state.clone());
    render::setup(state.clone(), canvas);
}
