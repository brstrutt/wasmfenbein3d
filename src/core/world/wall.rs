use std::{cell::RefCell, rc::Rc};

use super::painting::Painting;
use crate::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::tiling_texture::TilingTexture,
};

pub const WALL_HEIGHT: f64 = 2.0;

#[derive(Clone)]
pub struct Wall {
    pub position: Line2D,
    pub texture: Rc<RefCell<TilingTexture>>,
    pub painting: Painting,
}

impl Wall {
    pub fn new(line: Line2D, texture: &Rc<RefCell<TilingTexture>>, painting: Painting) -> Self {
        Wall {
            position: line,
            texture: texture.clone(),
            painting,
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

    pub fn get_texture_at_point(&self, wall_x_pos: f64) -> &Rc<RefCell<TilingTexture>> {
        if wall_x_pos < 2.0 {
            &self.painting.texture
        } else {
            &self.texture
        }
    }
}
