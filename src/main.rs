mod primitives;
mod render;
mod setup_render_loop;
mod world;
mod main_canvas;
mod controls;
mod log;

use setup_render_loop::setup_render_loop;
use std::{cell::RefCell, rc::Rc};

use crate::{main_canvas::MainCanvas, world::World};

fn main() {
    console_error_panic_hook::set_once();

    log::log("Starting up!");

    let mut canvas = MainCanvas::init();
    canvas.update_canvas_size();

    let world = Rc::new(RefCell::new(World::dummy()));

    controls::setup(world.clone());
    setup_render_loop(world.clone(), canvas);
}
