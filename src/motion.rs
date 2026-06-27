use crate::{
    primitives::{line2d::Line2D, point2d::Point2D},
    world::World,
};

pub fn move_object(start_position: Point2D, velocity: &Point2D, world: &World) -> Point2D {
    let new_position = &start_position + velocity;

    if !world.line_intersects_wall(&Line2D {
        start: start_position,
        end: new_position,
    }) {
        new_position
    } else {
        start_position
    }
}
