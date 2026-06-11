use crate::{main_canvas::MainCanvas, render::rgb::RGB};

pub fn _render(canvas: &MainCanvas, x: u32, y: u32, color: &RGB) {
    canvas.render_context.set_fill_style_str(format!("rgb({red},{green},{blue})", red = color.red, green = color.green, blue = color.blue).as_str());
    canvas.render_context.fill_rect(x as f64, y as f64, 1.0, 1.0);
}

pub fn render_column(canvas: &MainCanvas, x: u32, mut height: u32, color: &RGB) {
    if height > canvas.element.height() {
        height = canvas.element.height();
    }

    let center = canvas.element.height()/2;
    let half_height = height/2;
    let bottom = center - half_height;

    canvas.render_context.set_fill_style_str(format!("rgb({red},{green},{blue})", red = color.red, green = color.green, blue = color.blue).as_str());
    canvas.render_context.fill_rect(x as f64, bottom as f64, 1.0, f64::try_from(height).unwrap());
}

pub fn clear(canvas: &MainCanvas) {
    let half_height = f64::try_from(canvas.element.height()).unwrap() / 2.0;

    canvas.render_context.set_fill_style_str("rgb(30, 20, 10)");
    canvas.render_context.fill_rect(0.0, half_height, f64::try_from(canvas.element.width()).unwrap(), half_height);
    canvas.render_context.set_fill_style_str("rgb(30, 75, 130)");
    canvas.render_context.fill_rect(0.0, 0.0, f64::try_from(canvas.element.width()).unwrap(), half_height);
}
