use std::ops;

use crate::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::rgb::RGB,
};

pub struct Texture {
    texels: Vec<RGB>,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn get_default_wall() -> Self {
        const LIGHT_GREEN: RGB = RGB {
            red: 60,
            green: 175,
            blue: 30,
        };

        const DARK_GREEN: RGB = RGB {
            red: 30,
            green: 100,
            blue: 60,
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
            red: 60,
            green: 175,
            blue: 30,
        };

        const DARK_GREEN: RGB = RGB {
            red: 30,
            green: 100,
            blue: 60,
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

    pub fn get_texel(&self, x: usize, y: usize) -> &RGB {
        let x = x % self.width;
        let y = y % self.height;
        &self.texels[y * self.width + x]
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
