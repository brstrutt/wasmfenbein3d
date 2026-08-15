use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};

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
        .expect(
            format!(
                "Element with ID {} couldn't be converted into an HtmlButtonElement",
                id
            )
            .as_str(),
        )
}

pub fn main_canvas() -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id("screen_canvas")
        .expect("Couldn't find screen canvas element")
        .dyn_into::<HtmlCanvasElement>()
        .expect("Failed to convert canvas into HtmlCanvasElement")
}

pub fn main_canvas_context() -> web_sys::CanvasRenderingContext2d {
    main_canvas()
        .get_context("2d")
        .expect("Failed to get 2D context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("Failed to get 2D context even MORE")
}

pub fn popup_page() -> web_sys::HtmlElement {
    document()
        .get_element_by_id("pop_up_page")
        .expect("Couldn't find pop up page element")
        .dyn_into::<HtmlElement>()
        .expect("Failed to convert pop up page into HtmlElement")
}
