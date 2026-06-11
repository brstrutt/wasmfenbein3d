mod primitives;
mod render;
mod setup_render_loop;
mod world;
mod main_canvas;

use wasm_bindgen::JsValue;
use setup_render_loop::setup_render_loop;

use crate::{main_canvas::MainCanvas, world::{World, camera::Camera}};

fn main() {
    console_error_panic_hook::set_once();

    log("Starting up!");

    let mut canvas = MainCanvas::init();
    canvas.update_canvas_size();

    let world = World::dummy();
    let camera = Camera::dummy();

    setup_render_loop(world, canvas, camera);
}

fn log(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}
