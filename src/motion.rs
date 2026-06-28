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

    // Calculate the normal to the wall in the direction of the camera
    let wall_angle_radians = (intersection.wall.end - intersection.wall.start).get_angle();
    let wall_space_start_position = (start_position - intersection.wall.start).rotate(wall_angle_radians * -1.0);
    let wall_tangent = Point2D{x: 1.0 * wall_space_start_position.x.signum(), y: 0.0}.rotate(wall_angle_radians);

    // Calculate the new position by moving the intersection point out of the wall along the wall_tangent vector
    let moved_intersection_point = intersection.location + (wall_tangent * 0.001);

    return moved_intersection_point;
}
