use wasm_bindgen::convert::FromWasmAbi;
use web_sys::EventTarget;

use crate::web::add_event_listener_with_callback;

use super::access;

pub fn add_event_listener_with_callback<E: FromWasmAbi, T: FnMut(E)>(event_name: &str, run: T) {
    add_event_listener_with_callback::add_event_listener_with_callback(
        &mut EventTarget::from(access::document()),
        event_name,
        run,
    );
}
