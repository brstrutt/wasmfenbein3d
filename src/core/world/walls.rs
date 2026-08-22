use std::rc::Rc;

use super::wall::Wall;
use crate::core::{
    primitives::{line2d::Line2D, point2d::Point2D, ray2d::Ray2D},
    render::tiling_texture::TilingTexture,
};

pub fn walls_from_point_path(points: &[Point2D], texture: &Rc<TilingTexture>) -> Vec<Wall> {
    if points.len() < 2 {
        return vec![];
    }

    let mut lines = vec![];
    for index in 0..(points.len() - 1) {
        lines.push(Wall::new(
            Line2D {
                start: points[index],
                end: points[index + 1],
            },
            &texture,
            vec![],
        ));
    }
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
