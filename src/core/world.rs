pub mod camera;
pub mod wall;
pub mod walls;

use crate::core::{
    primitives::{line2d::Line2D, ray2d::Ray2D},
    render::textures::Textures,
    world::{camera::Camera, wall::Wall, walls::WallCollision},
};

#[derive(Clone)]
pub struct World {
    pub walls: Vec<Wall>,
    pub camera: Camera,
}

impl World {
    pub fn new(screen_width: usize, screen_height: usize, textures: &Textures) -> World {
        World {
            walls: walls::default_walls(textures),
            camera: Camera::new(screen_width, screen_height),
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
