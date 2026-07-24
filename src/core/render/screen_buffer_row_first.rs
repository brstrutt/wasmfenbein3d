use super::rgb::RGB;
use super::screen_buffer::ScreenBuffer;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

pub struct ScreenBufferRowFirst {
    pixels: Vec<u8>,
    already_drawn: Vec<bool>,
    width: usize,
    height: usize,
    row_pixel_index_increment: usize,
    column_pixel_index_increment: usize,
}

impl ScreenBufferRowFirst {
    pub fn setup(width: usize, height: usize) -> Self {
        ScreenBufferRowFirst {
            pixels: vec![255u8; width * height * 4],
            already_drawn: vec![false; width * height],
            width,
            height,
            row_pixel_index_increment: 1,
            column_pixel_index_increment: width,
        }
    }
}

impl ScreenBuffer for ScreenBufferRowFirst {
    fn reset_draw_history(&mut self) {
        self.already_drawn.fill(false)
    }

    #[inline(always)]
    fn render_pixel(&mut self, pixel_index: usize, colour: &RGB) {
        self.pixels[pixel_index << 2] = colour.red;
        self.pixels[(pixel_index << 2) + 1] = colour.green;
        self.pixels[(pixel_index << 2) + 2] = colour.blue;
    }

    #[inline(always)]
    fn mark_pixel_as_rendered(&mut self, pixel_index: usize) {
        self.already_drawn[pixel_index] = true;
    }

    #[inline(always)]
    fn pixel_drawn(&self, pixel_index: usize) -> &bool {
        &self.already_drawn[pixel_index]
    }

    #[inline(always)]
    fn row_pixel_index_increment(&self) -> usize {
        self.row_pixel_index_increment
    }

    #[inline(always)]
    fn column_pixel_index_increment(&self) -> usize {
        self.column_pixel_index_increment
    }

    #[inline(always)]
    fn width(&self) -> usize {
        self.width
    }

    #[inline(always)]
    fn height(&self) -> usize {
        self.height
    }

    #[inline(always)]
    fn coord_to_pixel_index(&self, x: &usize, y: &usize) -> usize {
        x + (y * self.column_pixel_index_increment)
    }

    fn to_imagedata(&self) -> ImageData {
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.pixels),
            self.width as u32,
            self.height as u32,
        )
        .expect("couldnt convert screen_buffer to ImageDats")
    }
}
