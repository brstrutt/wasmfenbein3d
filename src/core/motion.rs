use crate::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    world::World,
};

pub fn move_object(start_position: Point2D, velocity: &Point2D, world: &World) -> Point2D {
    if velocity.length().is_nan() || velocity.length() < 0.001 {
        return start_position;
    }

    let mut new_position = &start_position + velocity;
    let mut loops_remaining = 10;
    while let Some(intersection) = world.nearest_wall_intersecting_line(&Line2D {
        start: start_position,
        end: new_position,
    }) && loops_remaining > 0
    {
        loops_remaining -= 1;
        // Calculate the intended destination and original position relative to the Wall
        let wall_angle_radians = intersection.wall.get_angle();
        let wall_space_start_position =
            (start_position - intersection.wall.start).rotate(-wall_angle_radians);
        let wall_space_destination_point =
            (new_position - intersection.wall.start).rotate(-wall_angle_radians);

        // Move the destination to be right next to the wall on the same side as the start_position
        let wall_space_updated_dest = Point2D {
            x: 0.001 * wall_space_start_position.x.signum(),
            y: wall_space_destination_point.y,
        };

        // Move this updated destination back into world space
        new_position = wall_space_updated_dest.rotate(wall_angle_radians) + intersection.wall.start;
    }

    if loops_remaining < 1 {
        start_position
    } else {
        new_position
    }
}
