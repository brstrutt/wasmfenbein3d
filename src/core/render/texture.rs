use std::io::Cursor;

use image::ImageReader;

use super::rgb::WHITE;
use super::rgb_palette::RgbPalette;
use super::rgbv::RGBV;
use crate::core::primitives::{line2d::Line2D, point2d::Point2D};

#[derive(Clone)]
pub struct Texture {
    pub texels: Vec<RGBV>,
    pub width: usize,
    pub width_f64: f64,
    pub height: usize,
    pub height_f64: f64,
}

impl Texture {
    pub fn new_from_bmp_data(bmp_data: &[u8], palette: &mut RgbPalette) -> Self {
        let result = ImageReader::new(Cursor::new(bmp_data))
            .with_guessed_format()
            .expect("Failed to guess image format of raw data array.")
            .decode()
            .expect("Failed to decode image.");
        let width = result.width() as usize;
        let height = result.height() as usize;

        let bytes = result.to_rgb8();

        let mut texels = vec![RGBV::from_rgb(&WHITE, palette); width * height];
        let mut index = 0;
        for rgb in bytes.pixels() {
            texels[index] = RGBV::from_u8(&rgb.0, palette);
            index += 1;
        }

        Texture {
            texels,
            width,
            width_f64: width as f64,
            height,
            height_f64: height as f64,
        }
    }

    pub fn get_texel(&self, x: isize, y: isize) -> &RGBV {
        &self.texels[(y as usize * self.width) + x as usize]
    }

    pub fn get_texel_column_on_line_with_scale(
        &self,
        line: &Line2D,
        point: &Point2D,
        scale: f64,
    ) -> usize {
        let wall = line;
        let intersection = point;

        let wall_end_relative = wall.end - wall.start;
        let inverse_wall_angle = -wall_end_relative.get_angle();
        let wall_space_intersection = (*intersection - wall.start).rotate(inverse_wall_angle);

        let texture_x_pos = (wall_space_intersection.y / scale) * self.width as f64;
        texture_x_pos as usize
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn width_f64(&self) -> &f64 {
        &self.width_f64
    }

    pub fn height(&self) -> usize {
        self.height
    }
    pub fn height_f64(&self) -> &f64 {
        &self.height_f64
    }
}
