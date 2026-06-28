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
    pub fn get_default() -> Self {
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
            width: 4,
            height: 4,
            #[rustfmt::skip]
            texels: vec![
                LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN, DARK_GREEN,
                DARK_GREEN, LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN,
                LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN, DARK_GREEN,
                DARK_GREEN, LIGHT_GREEN, DARK_GREEN, LIGHT_GREEN,
            ],
        }
    }

    pub fn get_texel<'a>(&'a self, x: usize, y: usize) -> &'a RGB {
        let x = x % self.width;
        let y = y % self.height;
        &self.texels[y * self.width + x]
    }

    pub fn get_texel_column_on_line(&self, line: &Line2D, point: &Point2D) -> usize {
        let wall = line;
        let intersection = point;

        let wall_end_relative = wall.end - wall.start;
        let wall_angle = wall_end_relative.get_angle();
        let inverse_wall_angle = wall_angle * -1.0;
        let wall_space_end = wall_end_relative.rotate(inverse_wall_angle);
        let wall_space_intersection = (*intersection - wall.start).rotate(inverse_wall_angle);

        let texture_x_pos =
            (wall_space_intersection.y as f64 / wall_space_end.y as f64) * self.width as f64;
        texture_x_pos as usize
    }
}
