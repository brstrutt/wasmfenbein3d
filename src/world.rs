mod walls;
pub(crate) mod camera;

use crate::primitives::line2d::Line2D;

pub struct World {
    pub walls: Vec<Line2D>
}

impl World {
    pub fn dummy() -> World {
        World{walls: walls::default_walls()}
    }

    pub fn dist_to_wall(&self, raycast: &Line2D) -> Option<f64> {
        walls::dist_to_wall(&self.walls, raycast)
    }
}