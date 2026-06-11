mod primitives;
mod render;
mod setup_render_loop;
mod world;

use wasm_bindgen::JsValue;
use setup_render_loop::setup_render_loop;

use crate::{render::screen::Screen, world::{World, camera::Camera}};

fn main() {
    console_error_panic_hook::set_once();

    log("Starting up!");
    
    let screen = Screen::init();
    let world = World::dummy();
    let camera = Camera::dummy();

    setup_render_loop(world, screen, camera);
}

fn log(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}
