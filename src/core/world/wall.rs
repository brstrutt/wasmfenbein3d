use std::rc::Rc;

use super::painting::Painting;
use crate::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::{rgbv::RGBV, tiling_texture::TilingTexture},
};

pub const WALL_HEIGHT: f64 = 2.0;

#[derive(Clone)]
pub struct Wall {
    pub position: Line2D,
    pub texture: Rc<TilingTexture>,
    pub paintings: Vec<Painting>,
}

impl Wall {
    pub fn new(line: Line2D, texture: &Rc<TilingTexture>, paintings: Vec<Painting>) -> Self {
        Wall {
            position: line,
            texture: texture.clone(),
            paintings,
        }
    }

    pub fn get_wall_space_x_position(&self, world_space_intersection: &Point2D) -> f64 {
        let wall = self.position;
        let intersection = world_space_intersection;

        let wall_end_relative = wall.end - wall.start;
        let inverse_wall_angle = -wall_end_relative.get_angle();
        let wall_space_intersection = (*intersection - wall.start).rotate(inverse_wall_angle);

        wall_space_intersection.y
    }

    pub fn get_painting_indexes_in_column(&self, wall_space_x: f64) -> Vec<usize> {
        let mut paintings = vec![];
        for (index, painting) in self.paintings.iter().enumerate() {
            if wall_space_x >= painting.top_left_corner.x
                && wall_space_x <= painting.bottom_right_corner.x
            {
                paintings.push(index);
            }
        }
        paintings
    }

    pub fn get_wall_colour_or_painting_colour_at_position(
        &self,
        wall_space_x: f64,
        wall_space_y: f64,
        painting_indexes: &Vec<usize>,
    ) -> &RGBV {
        for index in painting_indexes.iter() {
            let painting = &self.paintings[*index];
            if wall_space_y >= painting.top_left_corner.y
                && wall_space_y <= painting.bottom_right_corner.y
            {
                return self.get_painting_texel(wall_space_x, wall_space_y, painting);
            }
        }

        return self.get_wall_texel(wall_space_x, wall_space_y);
    }

    pub fn get_wall_colour_at_position(&self, wall_space_x: f64, wall_space_y: f64) -> &RGBV {
        for painting in self.paintings.iter() {
            if wall_space_x >= painting.top_left_corner.x
                && wall_space_x <= painting.bottom_right_corner.x
                && wall_space_y >= painting.top_left_corner.y
                && wall_space_y <= painting.bottom_right_corner.y
            {
                return self.get_painting_texel(wall_space_x, wall_space_y, painting);
            }
        }

        return self.get_wall_texel(wall_space_x, wall_space_y);
    }

    fn get_wall_texel(&self, wall_space_x: f64, wall_space_y: f64) -> &RGBV {
        self.texture.get_texel(
            (wall_space_x * self.texture.width() as f64) as isize,
            (wall_space_y * self.texture.height() as f64 * WALL_HEIGHT) as isize,
        )
    }

    fn get_painting_texel<'a>(
        &'a self,
        wall_space_x: f64,
        wall_space_y: f64,
        painting: &'a Painting,
    ) -> &'a RGBV {
        let wall_space_x = wall_space_x - painting.top_left_corner.x;
        let wall_space_y = wall_space_y - painting.top_left_corner.y;
        let painting_height = painting.bottom_right_corner.y - painting.top_left_corner.y;
        let painting_width = painting.bottom_right_corner.x - painting.top_left_corner.x;

        painting.texture.get_texel(
            (wall_space_x * painting.texture.width() as f64 / painting_width) as isize,
            (wall_space_y * painting.texture.height() as f64 / painting_height) as isize,
        )
    }
}
