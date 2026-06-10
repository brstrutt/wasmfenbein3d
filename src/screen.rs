use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};
use crate::rgb::RGB;

pub struct Screen {
    pub width: u32,
    pub height: u32,
    canvas_context: CanvasRenderingContext2d,
}

impl Screen {
    pub fn init() -> Self {
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

        let width: u32 = u32::try_from(canvas_node.offset_width()).unwrap();
        let height: u32 = u32::try_from(canvas_node.offset_height()).unwrap();

        canvas_node.set_width(width);
        canvas_node.set_height(height);

        let canvas_context = canvas_node.get_context("2d")
            .expect("Failed to get 2D context")
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("Failed to get 2D context even MORE");

        Screen {
            width,
            height,
            canvas_context
        }
    }

    pub fn _render(&self, x: u32, y: u32, color: &RGB) {
        self.canvas_context.set_fill_style_str(format!("rgb({red},{green},{blue})", red = color.red, green = color.green, blue = color.blue).as_str());
        self.canvas_context.fill_rect(x as f64, y as f64, 1.0, 1.0);
    }

    pub fn render_column(&self, x: u32, mut height: u32, color: &RGB) {
        if height > self.height {
            height = self.height;
        }

        let center = self.height/2;
        let half_height = height/2;
        let bottom = center - half_height;

        self.canvas_context.set_fill_style_str(format!("rgb({red},{green},{blue})", red = color.red, green = color.green, blue = color.blue).as_str());
        self.canvas_context.fill_rect(x as f64, bottom as f64, 1.0, f64::try_from(height).unwrap());
    }

    pub fn clear(&self) {
        let half_height = f64::try_from(self.height).unwrap() / 2.0;

        self.canvas_context.set_fill_style_str("rgb(30, 20, 10)");
        self.canvas_context.fill_rect(0.0, half_height, f64::try_from(self.width).unwrap(), half_height);
        self.canvas_context.set_fill_style_str("rgb(30, 75, 130)");
        self.canvas_context.fill_rect(0.0, 0.0, f64::try_from(self.width).unwrap(), half_height);
    }
}
