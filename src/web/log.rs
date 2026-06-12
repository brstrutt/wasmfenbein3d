use wasm_bindgen::JsValue;

pub fn log(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}
