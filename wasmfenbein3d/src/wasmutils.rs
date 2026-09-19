use std::{cell::RefCell, rc::Rc};
use web_sys::{
    EventTarget,
    wasm_bindgen::{JsCast, closure::Closure, convert::FromWasmAbi},
};

pub fn now_in_ms() -> f64 {
    window()
        .performance()
        .expect("Couldnt get the window performance object")
        .now()
}

pub fn add_event_listener_with_callback<E: FromWasmAbi, T: FnMut(E)>(
    object: &mut EventTarget,
    event_name: &str,
    mut run: T,
) {
    let callback = Closure::wrap(Box::new(move |e: E| {
        run(e);
    }) as Box<dyn FnMut(_)>);
    object
        .add_event_listener_with_callback(event_name, callback.as_ref().unchecked_ref())
        .expect("Failed to setup event listener with custom rust lambda");
    callback.forget();
}

pub fn run_function_every_animation_frame<T: FnMut()>(mut run: T)
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
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window().document().expect("no global `document` exists")
}
