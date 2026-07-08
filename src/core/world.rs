pub mod camera;
pub mod walls;

use crate::core::{
    primitives::{line2d::Line2D, ray2d::Ray2D},
    world::{camera::Camera, walls::WallCollision},
};

#[derive(Clone)]
pub struct World {
    pub walls: Vec<Line2D>,
    pub camera: Camera,
}

impl World {
    pub fn new(screen_width: usize, screen_height: usize) -> World {
        World {
            walls: walls::default_walls(),
            camera: Camera::new(screen_width, screen_height),
        }
    }

    pub fn nearest_wall_intersecting_ray(&self, raycast: &Ray2D) -> Option<WallCollision> {
        walls::nearest_wall_intersection(&self.walls, raycast)
    }

    pub fn nearest_wall_intersecting_line(&self, line: &Line2D) -> Option<WallCollision> {
        walls::nearest_wall_intersecting_line(&self.walls, line)
    }
}
