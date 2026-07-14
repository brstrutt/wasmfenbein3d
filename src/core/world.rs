pub mod camera;
pub mod painting;
pub mod wall;
pub mod walls;

use std::{cell::RefCell, rc::Rc};

use crate::core::{
    primitives::{line2d::Line2D, ray2d::Ray2D},
    render::{rgb::WHITE, rgb_palette::RgbPalette, rgbv::RGBV, tiling_texture::TilingTexture},
    world::{camera::Camera, wall::Wall, walls::WallCollision},
};

#[derive(Clone)]
pub struct World {
    pub walls: Vec<Wall>,
    pub camera: Camera,
    pub skybox_colour: RGBV,
    pub floor: Rc<RefCell<TilingTexture>>,
    pub ceiling: Rc<RefCell<TilingTexture>>,
}

impl World {
    pub fn new(
        screen_width: usize,
        screen_height: usize,
        walls: Vec<Wall>,
        palette: &mut RgbPalette,
        floor: Rc<RefCell<TilingTexture>>,
        ceiling: Rc<RefCell<TilingTexture>>,
    ) -> World {
        World {
            walls: walls,
            camera: Camera::new(screen_width, screen_height),
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
