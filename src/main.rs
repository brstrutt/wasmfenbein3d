use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

fn main() {
    console_error_panic_hook::set_once();

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let canvas_node = document.create_element("canvas")
        .expect("Failed to create canvas node");
    body.append_child(canvas_node.as_ref())
        .expect("Failed to append canvas node");

    let canvas_node = canvas_node.dyn_into::<HtmlCanvasElement>()
        .expect("Failed to convert canvas into HtmlCanvasElement");

    let canvas_context = canvas_node.get_context("2d")
        .expect("Failed to get 2D context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("Failed to get 2D context even MORE");

    canvas_context.set_fill_style_str("#f54f00");
    canvas_context.fill_rect(10.0, 10.0, 100.0, 50.0);
}
