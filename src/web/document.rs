use wasm_bindgen::{JsCast, convert::FromWasmAbi, prelude::Closure};

use super::access;

pub fn add_event_listener_with_callback<E: FromWasmAbi, T: FnMut(E)>(event_name: &str, mut run: T) {
    let callback = Closure::wrap(Box::new(move |e: E| {
        run(e);
    }) as Box<dyn FnMut(_)>);
    access::document()
        .add_event_listener_with_callback(event_name, callback.as_ref().unchecked_ref())
        .expect("Failed to setup keydown event for controls");
    callback.forget();
}
