use crate::core::{primitives::point2d::Point2D, world::World};

pub struct InputState {
    pub move_left: bool,
    pub move_right: bool,
    pub move_forward: bool,
    pub move_backward: bool,
    pub sprint: bool,
    pub camera_rotation: i32,
    pub pointer_locked: bool,
    pub last_canvas_touch_point_x: Option<i32>,
    pub touch_has_moved_camera: bool,
}

impl InputState {
    pub fn setup() -> InputState {
        InputState {
            move_left: false,
            move_right: false,
            move_forward: false,
            move_backward: false,
            sprint: false,
            camera_rotation: 0,
            pointer_locked: false,
            last_canvas_touch_point_x: None,
            touch_has_moved_camera: false,
        }
    }

    pub fn get_cameraspace_movement_direction(&self) -> Point2D {
        let mut motion = Point2D { x: 0.0, y: 0.0 };
        if self.move_left {
            motion.x += 1.0;
        }
        if self.move_right {
            motion.x -= 1.0;
        }

        if self.move_forward {
            motion.y += 1.0;
        }
        if self.move_backward {
            motion.y -= 1.0;
        }
        motion.normalise()
    }

    pub fn get_items_under_cursor(&self, environment: &World) -> Vec<String> {
        let mut item_ids = vec![];
        if let Some(collision) = environment.nearest_wall_intersecting_ray(&environment.camera.ray)
        {
            if collision.wall.paintings.len() > 0 {
                for painting in collision.wall.get_paintings_in_column(
                    collision
                        .wall
                        .get_wall_space_x_position(&collision.intersection),
                ) {
                    item_ids.push(painting.id.clone());
                }
            };
        }
        item_ids
    }
}
