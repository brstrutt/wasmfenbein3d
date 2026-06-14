use crate::primitives::{line2d::Line2D, point2d::Point2D, ray2d::Ray2D};


pub fn default_walls() -> Vec<Line2D> {
    walls_from_point_path(&vec![
        Point2D::new(-5.0, -3.0),
        Point2D::new(-5.0, 5.0),
        Point2D::new(-1.0, 5.0),
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
    if points.len() < 2 { return vec![];}

    let mut lines = vec![];
    for index in 1..points.len() {
        lines.push(Line2D{start: points[index - 1], end: points[index]});
    }
    lines.push(Line2D{start: points[points.len() -1], end: points[0]});
    lines
}

pub fn dist_to_wall(walls: &Vec<Line2D>, raycast: &Ray2D) -> Option<f64> {
    let mut smallest_dist: Option<f64> = None;

    for wall in walls.iter() {
        let intersection_point = raycast.intersection(wall);
        if intersection_point.is_some() {
            let dist = Point2D::dist(&raycast.origin, &intersection_point.unwrap());
            if smallest_dist.is_none() || dist < smallest_dist.unwrap() {
                smallest_dist = Some(dist)
            }
        }
    }

    return smallest_dist;
}