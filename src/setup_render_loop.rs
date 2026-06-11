use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, prelude::Closure};
use crate::{
    main_canvas::MainCanvas, render::render, world::World
};

pub fn setup_render_loop(world: World, canvas: MainCanvas) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        // do the animation code here
        render(&canvas, &world);
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
