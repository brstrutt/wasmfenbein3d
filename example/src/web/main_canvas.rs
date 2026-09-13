use wasm_bindgen::Clamped;
use wasm_bindgen::convert::FromWasmAbi;
use wasmfenbein3d::render::screen_buffer_column_first::ScreenBufferColumnFirst;
use web_sys::EventTarget;
use web_sys::ImageData;

use super::access;
use super::add_event_listener_with_callback;
use wasmfenbein3d::render::screen_buffer::ScreenBuffer;

const CANVAS_SCALE: u32 = 2;

pub fn setup_screen_buffer() -> Box<dyn ScreenBuffer> {
    update_canvas_size();

    let screen_width = access::main_canvas().height() as usize;
    let screen_height = access::main_canvas().width() as usize;

    Box::new(ScreenBufferColumnFirst::setup(screen_width, screen_height))
}

fn update_canvas_size() {
    let element = access::main_canvas();
    let width: u32 = u32::try_from(element.offset_width()).unwrap();
    let height: u32 = u32::try_from(element.offset_height()).unwrap();

    element.set_width(width / CANVAS_SCALE);
    element.set_height(height / CANVAS_SCALE);
}

pub fn render_screen_buffer(screen_buffer: &Box<dyn ScreenBuffer>) {
    access::main_canvas_context()
        .put_image_data(&to_imagedata(screen_buffer), 0.0, 0.0)
        .expect("Failed to copy Screen Buffer to canvas.");
}

pub fn add_event_listener_with_callback<E: FromWasmAbi, T: FnMut(E)>(event_name: &str, run: T) {
    add_event_listener_with_callback::add_event_listener_with_callback(
        &mut EventTarget::from(access::main_canvas()),
        event_name,
        run,
    );
}

fn to_imagedata(screen_buffer: &Box<dyn ScreenBuffer>) -> ImageData {
    ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&screen_buffer.get_pixels()),
        screen_buffer.height() as u32,
        screen_buffer.width() as u32,
    )
    .expect("couldnt convert screen_buffer to ImageDats")
}
