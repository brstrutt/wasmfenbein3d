use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::window;

use crate::{log, world::World};


pub fn setup(world: Rc<RefCell<World>>) {
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");

    let callback = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        log::log(format!("Testing: {}", e.key()).as_str());
        match e.key().as_str() {
            "a" => world.borrow_mut().camera.position.x -= 1.0,
            "d" => world.borrow_mut().camera.position.x += 1.0,
            &_ => return,
        }
    }) as Box<dyn FnMut(_)>);
    document.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref()).expect("Failed to setup keydown event for controls");
    callback.forget();
}