use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlElement};

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window().document().expect("no global `document` exists")
}

pub fn main_canvas() -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id("screen_canvas")
        .expect("Couldn't find screen canvas element")
        .dyn_into::<HtmlCanvasElement>()
        .expect("Failed to convert canvas into HtmlCanvasElement")
}

pub fn popup_page() -> web_sys::HtmlElement {
    document()
        .get_element_by_id("pop_up_page")
        .expect("Couldn't find pop up page element")
        .dyn_into::<HtmlElement>()
        .expect("Failed to convert pop up page into HtmlElement")
}
