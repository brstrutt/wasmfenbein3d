mod rgb;
mod screen;
mod primitives;
use screen::Screen;
use rgb::RGB;
use wasm_bindgen::JsValue;

fn main() {
    console_error_panic_hook::set_once();

    log("Starting up!");
    let screen = Screen::init();

    log(format!("Screen ({},{})", screen.width, screen.height).as_str());

    for x in 0..=screen.width {
        for y in 0..=screen.height {
            screen.render(x, y, RGB {red: x, green: y, blue: (x + y) / 2});
        }
    }
}

fn log(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}
