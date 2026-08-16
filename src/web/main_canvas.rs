use std::cell::Ref;
use wasm_bindgen::convert::FromWasmAbi;
use web_sys::EventTarget;

use super::access;
use super::add_event_listener_with_callback;
use wasmfenbein3d::core::render::screen_buffer::ScreenBuffer;

const CANVAS_SCALE: u32 = 2;

pub fn update_canvas_size() {
    let element = access::main_canvas();
    let width: u32 = u32::try_from(element.offset_width()).unwrap();
    let height: u32 = u32::try_from(element.offset_height()).unwrap();

    element.set_width(width / CANVAS_SCALE);
    element.set_height(height / CANVAS_SCALE);
}

pub fn render_screen_buffer<Screen: ScreenBuffer>(screen_buffer: Ref<Screen>) {
    access::main_canvas_context()
        .put_image_data(&screen_buffer.to_imagedata(), 0.0, 0.0)
        .expect("Failed to copy Screen Buffer to canvas.");
}

pub fn add_event_listener_with_callback<E: FromWasmAbi, T: FnMut(E)>(event_name: &str, run: T) {
    add_event_listener_with_callback::add_event_listener_with_callback(
        &mut EventTarget::from(access::main_canvas()),
        event_name,
        run,
    );
}
