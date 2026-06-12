
pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window().document().expect("no global `document` exists")
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("no document body exists")
}