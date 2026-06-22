use wasm_bindgen::JsCast;


pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window().document().expect("no global `document` exists")
}

pub fn button(id: &str) -> web_sys::HtmlButtonElement {
    document()
        .get_element_by_id(id)
        .expect(format!("Couldn't find button element with ID: {}", id).as_str())
        .dyn_into::<web_sys::HtmlButtonElement>()
        .expect(format!("Element with ID {} couldn't be converted into an HtmlButtonElement", id).as_str())
}