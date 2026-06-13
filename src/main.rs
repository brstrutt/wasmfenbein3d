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

    let canvas = Rc::new(RefCell::new(MainCanvas::init()));
    {
        canvas.borrow_mut().update_canvas_size();
    }

    let state = Rc::new(RefCell::new(GameState::setup()));

    controls::setup(state.clone(), canvas.clone());
    render::setup(state.clone(), canvas.clone());
}
