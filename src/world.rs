use crate::primitives::line2d::_Line2D;
use crate::primitives::point2d::_Point2D;

pub struct World {
    pub walls: Vec<_Line2D>
}

impl World {
    pub fn dummy() -> World {
        World{walls: vec![
            _Line2D{start: _Point2D{ x: 10.0, y: 10.0 }, end: _Point2D { x: 10.0, y: 5.0 }},
            _Line2D{start: _Point2D{ x: 10.0, y: 10.0 }, end: _Point2D { x: 5.0, y: 10.0 }},
            _Line2D{start: _Point2D{ x: 5.0, y: 10.0 }, end: _Point2D { x: 5.0, y: 5.0 }},
            _Line2D{start: _Point2D{ x: 5.0, y: 5.0 }, end: _Point2D { x: 10.0, y: 5.0 }},
            _Line2D{start: _Point2D{ x: -7.0, y: -3.0 }, end: _Point2D { x: -2.0, y: -8.0 }},
            _Line2D{start: _Point2D{ x: 2.0, y: 2.0 }, end: _Point2D { x: 20.0, y: 20.0 }},
        ]}
    }

    pub fn dist_to_wall(&self, raycast: &_Line2D) -> Option<f64> {
        let mut smallest_dist: Option<f64> = None;

        for wall in self.walls.iter() {
            let intersection_point = raycast._intersection(wall);
            if intersection_point.is_some() {
                let dist = _Point2D::dist(&raycast.start, &intersection_point.unwrap());
                if smallest_dist.is_none() || dist < smallest_dist.unwrap() {
                    smallest_dist = Some(dist)
                }
            }
        }

        return smallest_dist;
    }
}