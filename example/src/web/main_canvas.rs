use wasmfenbein3d::render::screen_buffer_column_first::ScreenBufferColumnFirst;

use super::access;

const CANVAS_SCALE: u32 = 2;

pub fn setup_screen_buffer() -> ScreenBufferColumnFirst {
    update_canvas_size();

    let screen_width = access::main_canvas().height() as usize;
    let screen_height = access::main_canvas().width() as usize;

    ScreenBufferColumnFirst::setup(screen_width, screen_height)
}

fn update_canvas_size() {
    let element = access::main_canvas();
    let width: u32 = u32::try_from(element.offset_width()).unwrap();
    let height: u32 = u32::try_from(element.offset_height()).unwrap();

    element.set_width(width / CANVAS_SCALE);
    element.set_height(height / CANVAS_SCALE);
}
