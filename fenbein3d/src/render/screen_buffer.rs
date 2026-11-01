use super::rgb::RGB;

pub struct ScreenBuffer {
    pixels: Vec<u8>,
    already_drawn: Vec<bool>,
    width: usize,
    height: usize,
    row_pixel_index_increment: usize,
    column_pixel_index_increment: usize,
}

impl ScreenBuffer {
    pub fn setup(width: usize, height: usize) -> Self {
        ScreenBuffer {
            pixels: vec![255u8; width * height * 4],
            already_drawn: vec![false; width * height],
            width,
            height,
            row_pixel_index_increment: 1,
            column_pixel_index_increment: width,
        }
    }
}

impl ScreenBuffer {
    pub fn reset_draw_history(&mut self) {
        self.already_drawn.fill(false)
    }

    #[inline(always)]
    pub fn render_pixel(&mut self, pixel_index: usize, colour: &RGB) {
        self.pixels[pixel_index << 2] = colour.red;
        self.pixels[(pixel_index << 2) + 1] = colour.green;
        self.pixels[(pixel_index << 2) + 2] = colour.blue;
    }

    #[inline(always)]
    pub fn mark_pixel_as_rendered(&mut self, pixel_index: usize) {
        self.already_drawn[pixel_index] = true;
    }

    #[inline(always)]
    pub fn pixel_drawn(&self, pixel_index: usize) -> &bool {
        &self.already_drawn[pixel_index]
    }

    #[inline(always)]
    pub fn row_pixel_index_increment(&self) -> usize {
        self.row_pixel_index_increment
    }

    #[inline(always)]
    pub fn column_pixel_index_increment(&self) -> usize {
        self.column_pixel_index_increment
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline(always)]
    pub fn coord_to_pixel_index(&self, x: &usize, y: &usize) -> usize {
        x + (y * self.column_pixel_index_increment)
    }

    #[inline(always)]
    pub fn get_pixels(&self) -> &Vec<u8> {
        &self.pixels
    }
}
