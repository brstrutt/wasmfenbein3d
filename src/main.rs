mod rgb;
mod screen;
mod primitives;
mod world;

use std::{cell::RefCell, rc::Rc};

use screen::Screen;
use rgb::RGB;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

use crate::world::World;

fn main() {
    console_error_panic_hook::set_once();

    log("Starting up!");
    setup_render_loop();
}

fn render(screen: &Screen, i: &mut u32) {
    let timer_label = format!("Render run {}", i);
    web_sys::console::time_with_label(&timer_label);

    screen.clear();
    screen.render_column(10, 500, &RGB {red: 0, green: 0, blue: 0});
    for x in 11..=screen.width {
        screen.render_column(x, *i, &RGB {red: x/15, green: x / 10, blue: x / 20});
    }
    web_sys::console::time_end_with_label(&timer_label);

    *i = (*i + 1) % 500;
}

fn setup_render_loop() {
    let screen = Screen::init();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut i: u32 = 0;

    *g.borrow_mut() = Some(Closure::new(move || {
        // do the animation code here
        render(&screen, &mut i);
        // queue up another re-draw request
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    // queue up the first re-draw request, to start animation
    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn log(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}
