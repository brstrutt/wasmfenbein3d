use std::io::Cursor;

use image::ImageReader;

use super::colour::Colour;
use super::rgb::WHITE;
use super::rgb_palette::RgbPalette;
use super::rgbv::RGBV;
use super::texel_provider::TexelProvider;
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
        let mut x = 0;
        let mut y = 0;
        for rgb in bytes.pixels() {
            texels[x * height + y] = RGBV::from_u8(&rgb.0, palette);
            x += 1;
            if x >= width {
                x = 0;
                y += 1;
            }
        }

        Texture {
            texels,
            width,
            width_f64: width as f64,
            height,
            height_f64: height as f64,
        }
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
}

impl TexelProvider for Texture {
    fn get_texel(&self, x: isize, y: isize) -> &dyn Colour {
        #[cfg(debug_assertions)]
        {
            if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
                use crate::core::render::rgb;

                return &rgb::ERROR;
            }
        }

        &self.texels[(x as usize * self.height) + y as usize]
    }

    fn width(&self) -> usize {
        self.width
    }
    fn width_f64(&self) -> &f64 {
        &self.width_f64
    }

    fn height(&self) -> usize {
        self.height
    }
    fn height_f64(&self) -> &f64 {
        &self.height_f64
    }
}
