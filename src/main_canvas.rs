use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{render::screen::ScreenBuffer, web};

const CANVAS_SCALE: u32 = 2;

pub struct MainCanvas {
    pub element: HtmlCanvasElement,
    pub render_context: CanvasRenderingContext2d,
}

impl MainCanvas {
    pub fn init() -> Self {
        let document = web::access::document();

        let canvas_node = document
            .get_element_by_id("screen_canvas")
            .expect("Couldn't find screen canvas element")
            .dyn_into::<HtmlCanvasElement>()
            .expect("Failed to convert canvas into HtmlCanvasElement");

        canvas_node
            .style()
            .set_property("z-index", "-2")
            .expect("Failed to move the canvas into the background");

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

        self.element.set_width(width / CANVAS_SCALE);
        self.element.set_height(height / CANVAS_SCALE);
    }

    pub fn render_screen_buffer(&mut self, screen_buffer: &ScreenBuffer) {
        self.render_context
            .put_image_data(&screen_buffer.to_imagedata(), 0.0, 0.0)
            .expect("Failed to copy Screen Buffer to canvas.");
    }
}
