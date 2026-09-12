pub mod load_from_json;
pub mod painting;
pub mod wall;
pub mod walls;

use std::rc::Rc;

use crate::core::{
    primitives::{line2d::Line2D, ray2d::Ray2D},
    render::texel_provider::TexelProvider,
};

pub struct World {
    pub walls: Vec<wall::Wall>,
    pub floor: Rc<Box<dyn TexelProvider>>,
    pub ceiling: Rc<Box<dyn TexelProvider>>,
}

impl World {
    pub fn new(
        walls: Vec<wall::Wall>,
        floor: &Rc<Box<dyn TexelProvider>>,
        ceiling: &Rc<Box<dyn TexelProvider>>,
    ) -> World {
        World {
            walls: walls,
            floor: floor.clone(),
            ceiling: ceiling.clone(),
        }
    }

    pub fn nearest_wall_intersecting_ray<'a>(
        &'a self,
        raycast: &Ray2D,
    ) -> Option<walls::WallCollision<'a>> {
        walls::nearest_wall_intersection(&self.walls, raycast)
    }

    pub fn nearest_wall_intersecting_line<'a>(
        &'a self,
        line: &Line2D,
    ) -> Option<walls::WallCollision<'a>> {
        walls::nearest_wall_intersecting_line(&self.walls, line)
    }
}
