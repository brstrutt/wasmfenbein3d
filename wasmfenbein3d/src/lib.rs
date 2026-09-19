use std::{cell::RefCell, rc::Rc};

pub use fenbein3d::*;
use fenbein3d::{render::screen_buffer::ScreenBuffer, state::State};
use web_sys::wasm_bindgen::{Clamped, JsCast, closure::Closure};

pub fn setup_render<Screen: ScreenBuffer + 'static>(
    state: Rc<RefCell<State<Screen>>>,
    canvas: web_sys::HtmlCanvasElement,
) {
    run_function_every_animation_frame(move || {
        let render_start_time = now_in_ms();
        fenbein3d::render::render_to_screen_buffer(&state);
        let mut state = state.borrow_mut();
        copy_buffer_to_canvas(&state.screen_buffer, &canvas);
        let render_end_time = now_in_ms();

        state.stats.render_frame.last_duration_ms = render_end_time - render_start_time;
        state.stats.render_frame.last_time_ms = render_start_time;
    });
}

fn run_function_every_animation_frame<T: FnMut()>(mut run: T)
where
    T: 'static,
{
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        // do the animation code here
        run();
        // queue up another re-draw request
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    // queue up the first re-draw request, to start animation
    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn now_in_ms() -> f64 {
    web_sys::window()
        .expect("no global `window` exists")
        .performance()
        .expect("Couldnt get the window performance object")
        .now()
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
