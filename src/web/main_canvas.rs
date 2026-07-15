use super::access;
use wasmfenbein3d::core::render::screen_buffer::ScreenBuffer;

const CANVAS_SCALE: u32 = 2;

pub fn setup() {
    access::main_canvas()
        .style()
        .set_property("z-index", "-2")
        .expect("Failed to move the canvas into the background");
}

pub fn update_canvas_size() {
    let element = access::main_canvas();
    let width: u32 = u32::try_from(element.offset_width()).unwrap();
    let height: u32 = u32::try_from(element.offset_height()).unwrap();

    element.set_width(width / CANVAS_SCALE);
    element.set_height(height / CANVAS_SCALE);
}

pub fn render_screen_buffer(screen_buffer: &ScreenBuffer) {
    access::main_canvas_context()
        .put_image_data(&screen_buffer.to_imagedata(), 0.0, 0.0)
        .expect("Failed to copy Screen Buffer to canvas.");
}
