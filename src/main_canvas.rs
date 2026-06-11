use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

pub struct MainCanvas {
    pub element: HtmlCanvasElement,
    pub render_context: CanvasRenderingContext2d,
}

impl MainCanvas {
    pub fn init() -> Self {
        let document = window()
            .and_then(|win| win.document())
            .expect("Could not access the document");
        let body = document.body().expect("Could not access document.body");

        let canvas_node = document
            .create_element("canvas")
            .expect("Failed to create canvas node");

        body.append_child(canvas_node.as_ref())
            .expect("Failed to append canvas node");

        let canvas_node = canvas_node
            .dyn_into::<HtmlCanvasElement>()
            .expect("Failed to convert canvas into HtmlCanvasElement");

        let canvas_context = canvas_node
            .get_context("2d")
            .expect("Failed to get 2D context")
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("Failed to get 2D context even MORE");



        MainCanvas {
            element: canvas_node,
            render_context: canvas_context,
        }
    }

    pub fn update_canvas_size(&mut self) {
        let width: u32 = u32::try_from(self.element.offset_width()).unwrap();
        let height: u32 = u32::try_from(self.element.offset_height()).unwrap();

        self.element.set_width(width);
        self.element.set_height(height);
    }
}
