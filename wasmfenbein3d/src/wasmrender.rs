use fenbein3d::{render::screen_buffer::ScreenBuffer, state::State};
use std::{cell::RefCell, rc::Rc};
use web_sys::wasm_bindgen::{Clamped, JsCast};

use crate::wasmutils;

pub fn setup<Screen: ScreenBuffer + 'static>(
    state: Rc<RefCell<State<Screen>>>,
    canvas: web_sys::HtmlCanvasElement,
) {
    wasmutils::run_function_every_animation_frame(move || {
        let render_start_time = wasmutils::now_in_ms();
        fenbein3d::render::render_to_screen_buffer(&state);
        let mut state = state.borrow_mut();
        copy_buffer_to_canvas(&state.screen_buffer, &canvas);
        let render_end_time = wasmutils::now_in_ms();

        state.stats.render_frame.last_duration_ms = render_end_time - render_start_time;
        state.stats.render_frame.last_time_ms = render_start_time;
    });
}

fn copy_buffer_to_canvas<Screen: ScreenBuffer>(
    screen_buffer: &Screen,
    canvas: &web_sys::HtmlCanvasElement,
) {
    canvas
        .get_context("2d")
        .expect("Failed to get 2D context")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Failed to get 2D context even MORE")
        .put_image_data(&to_imagedata(screen_buffer), 0.0, 0.0)
        .expect("Failed to copy Screen Buffer to canvas.");
}

fn to_imagedata<Screen: ScreenBuffer>(screen_buffer: &Screen) -> web_sys::ImageData {
    web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&screen_buffer.get_pixels()),
        screen_buffer.height() as u32,
        screen_buffer.width() as u32,
    )
    .expect("couldnt convert screen_buffer to ImageDats")
}
