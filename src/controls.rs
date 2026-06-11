use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{KeyboardEvent, window};

use crate::world::World;


pub fn setup(world: Rc<RefCell<World>>) {
    run_function_on_document_keydown(move |e: KeyboardEvent| {
        match e.key().as_str() {
            "a" => world.borrow_mut().camera.position.x -= 10.0,
            "d" => world.borrow_mut().camera.position.x += 10.0,
            &_ => return,
        }
    });
}

fn run_function_on_document_keydown<T: FnMut(KeyboardEvent)>(mut run: T) where T: 'static{
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");

    let callback = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        run(e);
    }) as Box<dyn FnMut(_)>);
    document.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref()).expect("Failed to setup keydown event for controls");
    callback.forget();
}