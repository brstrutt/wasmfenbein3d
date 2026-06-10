use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;

pub struct World {
    pub walls: Vec<Line2D>
}

impl World {
    pub fn dummy() -> World {
        World{walls: vec![
            Line2D{start: Point2D{ x: 10.0, y: 10.0 }, end: Point2D { x: 10.0, y: 5.0 }},
            Line2D{start: Point2D{ x: 2.0, y: 2.0 }, end: Point2D { x: 200.0, y: 200.0 }},
            Line2D{start: Point2D{ x: 200.0, y: 2.0 }, end: Point2D { x: 500.0, y: 200.0 }},
            Line2D{start: Point2D{ x: 1000.0, y: 2.0 }, end: Point2D { x: 500.0, y: 200.0 }},
        ]}
    }

    pub fn dist_to_wall(&self, raycast: &Line2D) -> Option<f64> {
        let mut smallest_dist: Option<f64> = None;

        for wall in self.walls.iter() {
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
}