use super::rgb::RGB;
use web_sys::ImageData;

pub trait ScreenBuffer {
    fn reset_draw_history(&mut self);

    fn render_pixel(&mut self, pixel_index: usize, colour: &RGB);

    fn mark_pixel_as_rendered(&mut self, pixel_index: usize);

    fn pixel_drawn(&self, pixel_index: usize) -> &bool;

    fn row_pixel_index_increment(&self) -> usize;
    fn column_pixel_index_increment(&self) -> usize;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn coord_to_pixel_index(&self, x: &usize, y: &usize) -> usize;

    fn to_imagedata(&self) -> ImageData;
}
