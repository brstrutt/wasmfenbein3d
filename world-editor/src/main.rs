use wasm_bindgen::prelude::*;

fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or_else(|| JsValue::from_str("The browser document is unavailable"))?;
    let app = document
        .get_element_by_id("app")
        .ok_or_else(|| JsValue::from_str("The app root element is missing"))?;

    app.set_inner_html("<h1>World Editor</h1><p>Your world editing workspace is ready.</p>");

    Ok(())
}
