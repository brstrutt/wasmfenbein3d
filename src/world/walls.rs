use crate::primitives::{line2d::Line2D, point2d::Point2D, ray2d::Ray2D};

pub const WALL_HEIGHT: f64 = 2.0;

pub fn default_walls() -> Vec<Line2D> {
    walls_from_point_path(&vec![
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
    ])
}

fn walls_from_point_path(points: &Vec<Point2D>) -> Vec<Line2D> {
    if points.len() < 2 {
        return vec![];
    }

    let mut lines = vec![];
    for index in 1..points.len() {
        lines.push(Line2D {
            start: points[index - 1],
            end: points[index],
        });
    }
    lines.push(Line2D {
        start: points[points.len() - 1],
        end: points[0],
    });
    lines
}

pub struct WallCollision {
    pub intersection: Point2D,
    pub wall: Line2D,
}

pub fn nearest_wall_intersection(walls: &Vec<Line2D>, raycast: &Ray2D) -> Option<WallCollision> {
    let mut closest_collision_distance: Option<f64> = None;
    let mut closest_collision: Option<WallCollision> = None;

    for wall in walls.iter() {
        let intersection_point = raycast.intersection(wall);
        if let Some(intersection_point) = intersection_point {
            let dist = Point2D::dist(&raycast.origin, &intersection_point);
            if closest_collision_distance.is_none() || dist < closest_collision_distance.unwrap() {
                closest_collision_distance = Some(dist);
                closest_collision = Some(WallCollision {
                    intersection: intersection_point,
                    wall: wall.clone(),
                })
            }
        }
    }

    return closest_collision;
}

pub fn line_intersects_wall(walls: &Vec<Line2D>, line: &Line2D) -> Option<WallCollision> {
    for wall in walls.iter() {
        let intersection_point = line.intersection(wall);
        if let Some(intersection_point) = intersection_point {
            return Some(WallCollision {
                intersection: intersection_point,
                wall: wall.clone(),
            });
        }
    }

    return None;
}
