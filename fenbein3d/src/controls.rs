use crate::{primitives::point2d::Point2D, render::screen_buffer::ScreenBuffer, state::State};

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

#[derive(PartialEq, Clone)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

#[derive(PartialEq)]
pub enum MovementEvent {
    Start,
    Stop,
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

    pub fn get_items_under_cursor<Screen: ScreenBuffer>(
        &self,
        state: &State<Screen>,
    ) -> Vec<String> {
        let mut item_ids = vec![];
        if let Some(collision) = state.world.nearest_wall_intersecting_ray(&state.camera.ray) {
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

    pub fn reset_movement(&mut self) {
        self.sprint = false;
        self.move_left = false;
        self.move_right = false;
        self.move_forward = false;
        self.move_backward = false;
    }

    pub fn change_direction(&mut self, direction: &Direction, event_type: MovementEvent) {
        let start_move = event_type == MovementEvent::Start;
        match direction {
            Direction::Left => self.move_left = start_move,
            Direction::Right => self.move_right = start_move,
            Direction::Forward => self.move_forward = start_move,
            Direction::Backward => self.move_backward = start_move,
        }
    }
}
