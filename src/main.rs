mod primitives;
mod render;
mod world;
mod main_canvas;
mod controls;
mod web;

use std::{cell::RefCell, rc::Rc};

use crate::{main_canvas::MainCanvas, world::World};

fn main() {
    console_error_panic_hook::set_once();

    web::log::log("Starting up!");

    let mut canvas = MainCanvas::init();
    canvas.update_canvas_size();

    let world = Rc::new(RefCell::new(World::dummy()));

    controls::setup(world.clone());
    render::setup(world.clone(), canvas);
}
