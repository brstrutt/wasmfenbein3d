use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{KeyboardEvent};

use crate::web;

pub fn add_event_listener_with_callback<T: FnMut(KeyboardEvent)>(mut run: T) where T: 'static{
    let callback = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        run(e);
    }) as Box<dyn FnMut(_)>);
    web::access::document().add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref()).expect("Failed to setup keydown event for controls");
    callback.forget();
}
