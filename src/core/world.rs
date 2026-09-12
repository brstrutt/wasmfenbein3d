pub mod painting;
pub mod wall;
pub mod walls;

use std::rc::Rc;

use crate::core::{
    primitives::{line2d::Line2D, ray2d::Ray2D},
    render::{rgb::WHITE, rgb_palette::RgbPalette, rgbv::RGBV, tiling_texture::TilingTexture},
    world::{wall::Wall, walls::WallCollision},
};

pub struct World {
    pub walls: Vec<Wall>,
    pub skybox_colour: RGBV,
    pub floor: Rc<TilingTexture>,
    pub ceiling: Rc<TilingTexture>,
}

impl World {
    pub fn new(
        walls: Vec<Wall>,
        palette: &mut RgbPalette,
        floor: Rc<TilingTexture>,
        ceiling: Rc<TilingTexture>,
    ) -> World {
        World {
            walls: walls,
            skybox_colour: RGBV::from_rgb(&WHITE, palette),
            floor,
            ceiling,
        }
    }

    pub fn nearest_wall_intersecting_ray<'a>(
        &'a self,
        raycast: &Ray2D,
    ) -> Option<WallCollision<'a>> {
        walls::nearest_wall_intersection(&self.walls, raycast)
    }

    pub fn nearest_wall_intersecting_line<'a>(
        &'a self,
        line: &Line2D,
    ) -> Option<WallCollision<'a>> {
        walls::nearest_wall_intersecting_line(&self.walls, line)
    }
}
