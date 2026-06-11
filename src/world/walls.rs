use crate::primitives::{line2d::Line2D, point2d::Point2D};


pub fn default_walls() -> Vec<Line2D> {
    vec![
        Line2D{start: Point2D{ x: 10.0, y: 10.0 }, end: Point2D { x: 10.0, y: 5.0 }},
        Line2D{start: Point2D{ x: 2.0, y: 2.0 }, end: Point2D { x: 200.0, y: 200.0 }},
        Line2D{start: Point2D{ x: 200.0, y: 2.0 }, end: Point2D { x: 400.0, y: 200.0 }},
        Line2D{start: Point2D{ x: 1000.0, y: 2.0 }, end: Point2D { x: 600.0, y: 200.0 }},
        Line2D{start: Point2D{ x: 1000.0, y: 2.0 }, end: Point2D { x: 1500.0, y: 200.0 }},
    ]
}

pub fn dist_to_wall(walls: &Vec<Line2D>, raycast: &Line2D) -> Option<f64> {
    let mut smallest_dist: Option<f64> = None;

    for wall in walls.iter() {
        let intersection_point = raycast.intersection(wall);
        if intersection_point.is_some() {
            let dist = Point2D::dist(&raycast.start, &intersection_point.unwrap());
            if smallest_dist.is_none() || dist < smallest_dist.unwrap() {
                smallest_dist = Some(dist)
            }
        }
    }

    return smallest_dist;
}