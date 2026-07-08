use std::{io::Cursor, ops};

use image::ImageReader;

use crate::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::rgb::{self, RGB},
};

pub struct Texture {
    pub texels: Vec<RGB>,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn new_from_bmp_data(bmp_data: &[u8]) -> Self {
        let result = ImageReader::new(Cursor::new(bmp_data))
            .with_guessed_format()
            .expect("Failed to guess image format of raw data array.")
            .decode()
            .expect("Failed to decode image.");
        let width = result.width() as usize;
        let height = result.height() as usize;

        let bytes = result.to_rgb8();

        let mut texels = vec![rgb::WHITE; width * height];
        let mut index = 0;
        for rgb in bytes.pixels() {
            texels[index] = RGB::from_u8(&rgb.0);
            index += 1;
        }

        Texture {
            width,
            height,
            texels,
        }
    }

    pub fn get_texel(&self, x: isize, y: isize) -> &RGB {
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
}

impl ops::Div<f64> for &Texture {
    type Output = Texture;

    fn div(self, rhs: f64) -> Texture {
        Texture {
            texels: self
                .texels
                .clone()
                .into_iter()
                .map(|texel| texel / rhs)
                .collect(),
            width: self.width,
            height: self.height,
        }
    }
}
