use std::{io::Cursor, ops};

use image::ImageReader;

use crate::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::rgb::RGB,
    utils::wrapping_mod::wrapping_mod,
};

pub struct Texture {
    texels: Vec<RGB>,
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

        let mut texels = vec![RGB::white(); width * height];
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

    pub fn get_default_wall() -> Self {
        const LIGHT_GREEN: RGB = RGB {
            red: 60,
            green: 175,
            blue: 100,
        };

        const DARK_GREEN: RGB = RGB {
            red: 30,
            green: 100,
            blue: 80,
        };

        Texture {
            width: 8,
            height: 8,
            #[rustfmt::skip]
            texels: vec![
                LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN, DARK_GREEN, DARK_GREEN, DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN,
                DARK_GREEN, DARK_GREEN, DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN, DARK_GREEN, DARK_GREEN,
                LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN,
                LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN,
                DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN,
                DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN, DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN,
                LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN, DARK_GREEN, DARK_GREEN, LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN,
                LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN, DARK_GREEN, DARK_GREEN, DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN,
            ],
        }
    }

    pub fn get_default_floor() -> Self {
        const LIGHT_GREEN: RGB = RGB {
            red: 100,
            green: 120,
            blue: 90,
        };

        const DARK_GREEN: RGB = RGB {
            red: 50,
            green: 70,
            blue: 70,
        };

        Texture {
            width: 8,
            height: 8,
            #[rustfmt::skip]
            texels: vec![
                LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN, DARK_GREEN, DARK_GREEN, DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN,
                LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN,
                DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN,
                DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN,
                DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN,
                DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN,
                LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN,
                LIGHT_GREEN, LIGHT_GREEN, DARK_GREEN, DARK_GREEN, DARK_GREEN, DARK_GREEN, LIGHT_GREEN, LIGHT_GREEN,
            ],
        }
    }

    pub fn get_texel(&self, x: isize, y: isize) -> &RGB {
        let x = wrapping_mod(x, self.width as isize) as usize;
        let y = wrapping_mod(y, self.height as isize) as usize;
        &self.texels[(y * self.width) + x]
    }

    pub fn get_texel_column(&self, x: usize) -> Texture {
        let x = x % self.width;
        let mut texels_slice = vec![
            RGB {
                red: 0,
                green: 0,
                blue: 0
            };
            self.height
        ];
        for (i, sub_texel) in texels_slice.iter_mut().enumerate() {
            *sub_texel = self.texels[x + (i * self.width)].clone();
        }
        Texture {
            width: 1,
            height: self.height,
            texels: texels_slice,
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
