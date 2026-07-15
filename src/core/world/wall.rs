use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use super::painting::Painting;
use crate::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::{rgbv::RGBV, tiling_texture::TilingTexture},
};

pub const WALL_HEIGHT: f64 = 2.0;

#[derive(Clone)]
pub struct Wall {
    pub position: Line2D,
    pub texture: Rc<RefCell<TilingTexture>>,
    pub painting: Option<Painting>,
}

impl Wall {
    pub fn new(
        line: Line2D,
        texture: &Rc<RefCell<TilingTexture>>,
        painting: Option<Painting>,
    ) -> Self {
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

    pub fn get_wall_colour_at_position(
        &self,
        wall_space_x: f64,
        wall_space_y: f64,
    ) -> Ref<'_, RGBV> {
        if let Some(painting) = &self.painting {
            if wall_space_x >= painting.top_left_corner.x
                && wall_space_x <= painting.bottom_right_corner.x
                && wall_space_y >= painting.top_left_corner.y
                && wall_space_y <= painting.bottom_right_corner.y
            {
                let wall_space_x = wall_space_x - painting.top_left_corner.x;
                let wall_space_y = wall_space_y - painting.top_left_corner.y;
                let painting_height = painting.bottom_right_corner.y - painting.top_left_corner.y;
                let painting_width = painting.bottom_right_corner.x - painting.top_left_corner.x;

                return Ref::map(painting.texture.borrow(), |tex| {
                    let texture_x_pos = wall_space_x * tex.width() as f64 / painting_width;
                    let texture_y_pos = wall_space_y * tex.height() as f64 / painting_height;
                    tex.get_texel(texture_x_pos as isize, texture_y_pos as isize)
                });
            }
        }

        return Ref::map(self.texture.borrow(), |tex| {
            let texture_y_pos = (wall_space_y * tex.height() as f64 * WALL_HEIGHT) as isize;
            tex.get_texel(
                (wall_space_x * tex.width() as f64) as isize,
                texture_y_pos as isize,
            )
        });
    }
}
