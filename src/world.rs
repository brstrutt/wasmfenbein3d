mod walls;
pub(crate) mod camera;

use crate::{primitives::{line2d::Line2D, ray2d::Ray2D}, world::camera::Camera};

#[derive(Clone)]
pub struct World {
    pub walls: Vec<Line2D>,
    pub camera: Camera,
}

impl World {
    pub fn dummy() -> World {
        World{
            walls: walls::default_walls(),
            camera: Camera::dummy(),
        }
    }

    pub fn dist_to_wall(&self, raycast: &Ray2D) -> Option<f64> {
        walls::dist_to_wall(&self.walls, raycast)
    }

    pub fn line_intersects_wall(&self, line: &Line2D) -> bool {
        walls::line_intersects_wall(&self.walls, line)
    }
}