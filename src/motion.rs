use crate::{
    primitives::{line2d::Line2D, point2d::Point2D}, world::World,
};

pub fn move_object(start_position: Point2D, velocity: &Point2D, world: &World) -> Point2D {
    if velocity.length().is_nan() || velocity.length() < 0.001 {
        return start_position;
    }

    let new_position = &start_position + velocity;
    let intersection = world.line_intersects_wall(&Line2D {
        start: start_position,
        end: new_position,
    });

    if intersection.is_none() {
        return new_position;
    }

    let intersection = intersection.unwrap();

    // Calculate the intended destination and original position relative to the Wall
    let wall_angle_radians = (intersection.wall.end - intersection.wall.start).get_angle();
    let wall_space_start_position = (start_position - intersection.wall.start).rotate(wall_angle_radians * -1.0);
    let wall_space_destination_point = (new_position - intersection.wall.start).rotate(wall_angle_radians * -1.0);

    // Move the destination to be right next to the wall on the same side as the start_position
    let wall_space_updated_dest = Point2D{x: 0.001 * wall_space_start_position.x.signum(), y: wall_space_destination_point.y};


    // Move this updated destination back into world space
    let updated_dest = wall_space_updated_dest.rotate(wall_angle_radians) + intersection.wall.start;

    return updated_dest;
}
