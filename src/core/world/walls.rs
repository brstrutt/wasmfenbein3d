use super::wall::Wall;
use crate::core::{
    primitives::{line2d::Line2D, point2d::Point2D, ray2d::Ray2D},
    render::textures::Textures,
};

pub const WALL_HEIGHT: f64 = 2.0;

pub fn default_walls(textures: &Textures) -> Vec<Wall> {
    walls_from_point_path(
        &vec![
            Point2D::new(-5.0, -3.0),
            Point2D::new(-5.0, 5.0),
            Point2D::new(-1.0, 5.0),
            Point2D::new(-1.0, 20.0),
            Point2D::new(8.0, 20.0),
            Point2D::new(8.0, 23.0),
            Point2D::new(15.0, 23.0),
            Point2D::new(15.0, 15.0),
            Point2D::new(8.0, 15.0),
            Point2D::new(8.0, 18.0),
            Point2D::new(1.0, 18.0),
            Point2D::new(1.0, 5.0),
            Point2D::new(5.0, 5.0),
            Point2D::new(5.0, -5.0),
            Point2D::new(-10.0, -5.0),
            Point2D::new(-10.0, -7.0),
            Point2D::new(-13.0, -7.0),
            Point2D::new(-13.0, -1.0),
            Point2D::new(-10.0, -1.0),
            Point2D::new(-10.0, -3.0),
        ],
        textures,
    )
}

fn walls_from_point_path(points: &[Point2D], textures: &Textures) -> Vec<Wall> {
    if points.len() < 2 {
        return vec![];
    }

    let mut lines = vec![];
    for index in 1..points.len() {
        lines.push(Wall::new(
            Line2D {
                start: points[index - 1],
                end: points[index],
            },
            &textures.floor,
        ));
    }
    lines.push(Wall::new(
        Line2D {
            start: points[points.len() - 1],
            end: points[0],
        },
        &textures.wall,
    ));
    lines
}

pub struct WallCollision<'a> {
    pub intersection: Point2D,
    pub wall: &'a Wall,
}

pub fn nearest_wall_intersection<'a>(
    walls: &'a [Wall],
    raycast: &Ray2D,
) -> Option<WallCollision<'a>> {
    let mut closest_collision_distance: Option<f64> = None;
    let mut closest_collision: Option<WallCollision> = None;

    for wall in walls.iter() {
        let intersection_point = raycast.intersection(&wall.position);
        if let Some(intersection_point) = intersection_point {
            let dist = Point2D::dist(&raycast.origin, &intersection_point);
            if closest_collision_distance.is_none() || dist < closest_collision_distance.unwrap() {
                closest_collision_distance = Some(dist);
                closest_collision = Some(WallCollision {
                    intersection: intersection_point,
                    wall,
                })
            }
        }
    }

    closest_collision
}

pub fn nearest_wall_intersecting_line<'a>(
    walls: &'a [Wall],
    line: &Line2D,
) -> Option<WallCollision<'a>> {
    let mut closest_collision_distance: Option<f64> = None;
    let mut closest_collision: Option<WallCollision> = None;

    for wall in walls.iter() {
        let intersection_point = line.intersection(&wall.position);
        if let Some(intersection_point) = intersection_point {
            let dist = Point2D::dist(&line.start, &intersection_point);
            if closest_collision_distance.is_none() || dist < closest_collision_distance.unwrap() {
                closest_collision_distance = Some(dist);
                closest_collision = Some(WallCollision {
                    intersection: intersection_point,
                    wall,
                });
            }
        }
    }

    closest_collision
}
