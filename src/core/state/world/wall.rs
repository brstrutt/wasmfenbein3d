use std::rc::Rc;

use super::painting::Painting;
use crate::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::tiling_texture::TilingTexture,
};

pub const WALL_HEIGHT: f64 = 2.0;

pub struct Wall {
    pub position: Line2D,
    pub texture: Rc<TilingTexture>,
    pub paintings: Vec<Painting>,
}

impl Wall {
    pub fn new(line: Line2D, texture: &Rc<TilingTexture>, mut paintings: Vec<Painting>) -> Self {
        paintings.sort_by(|a, b| a.top_left_corner.y.total_cmp(&b.top_left_corner.y));
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

    pub fn get_paintings_in_column(&self, wall_space_x: f64) -> Vec<&Painting> {
        let mut paintings = vec![];
        for painting in self.paintings.iter() {
            if wall_space_x >= painting.top_left_corner.x
                && wall_space_x <= painting.bottom_right_corner.x
            {
                paintings.push(painting);
            }
        }
        paintings
    }
}
