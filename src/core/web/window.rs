use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, prelude::Closure};

use crate::core::web;

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

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web::access::window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn now_in_ms() -> f64 {
    web::access::window()
        .performance()
        .expect("Couldnt get the window performance object")
        .now()
}
