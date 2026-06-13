use std::{cell::RefCell, rc::Rc};
use web_sys::KeyboardEvent;

use crate::{state::GameState, web};

pub struct InputState {
    pub move_left: bool,
    pub move_right: bool,
    pub move_forward: bool,
    pub move_backward: bool,
    pub rotate_camera_left: bool,
    pub rotate_camera_right: bool,
}

impl InputState {
    pub fn setup() -> InputState {
        InputState {
            move_left: false,
            move_right: false,
            move_forward: false,
            move_backward: false,
            rotate_camera_left: false,
            rotate_camera_right: false,
        }
    }
}

pub fn setup(state: Rc<RefCell<GameState>>) {
    {
        let state = state.clone();
        web::document::add_event_listener_with_callback("keydown", move |e: KeyboardEvent| {
            let mut state = state.borrow_mut();
            match e.key().as_str() {
                "a" | "A" => state.input.move_left = true,
                "d" | "D" => state.input.move_right = true,
                "w" | "W" => state.input.move_forward = true,
                "s" | "S" => state.input.move_backward = true,
                "ArrowRight" => state.input.rotate_camera_left = true,
                "ArrowLeft" => state.input.rotate_camera_right = true,
                &_ => return,
            }
        });
    }

    {
        let state = state.clone();
        web::document::add_event_listener_with_callback("keyup", move |e: KeyboardEvent| {
            let mut state = state.borrow_mut();
            match e.key().as_str() {
                "a" | "A" => state.input.move_left = false,
                "d" | "D" => state.input.move_right = false,
                "w" | "W" => state.input.move_forward = false,
                "s" | "S" => state.input.move_backward = false,
                "ArrowRight" => state.input.rotate_camera_left = false,
                "ArrowLeft" => state.input.rotate_camera_right = false,
                &_ => return,
            }
        });
    }

    {
        let state = state.clone();
        web::window::run_function_every_animation_frame(move || {
            let mut state = state.borrow_mut();
            const SPEED: f64 = 0.1; // move 0.1 per frame

            let mut sideways_move = 0;
            if state.input.move_left {
                sideways_move -= 1;
            }
            if state.input.move_right {
                sideways_move += 1;
            }

            let mut forwards_move = 0;
            if state.input.move_forward {
                forwards_move += 1;
            }
            if state.input.move_backward {
                forwards_move -= 1;
            }

            let mut camera_rotation = 0;
            if state.input.rotate_camera_left {
                camera_rotation += 1;
            }
            if state.input.rotate_camera_right {
                camera_rotation -= 1;
            }

            if sideways_move != 0 {
                let move_right_direction = state
                    .world
                    .camera
                    .rotate(std::f32::consts::PI as f64 / 2.0)
                    .direction;
                state.world.camera.origin = state.world.camera.origin
                    + (move_right_direction * sideways_move as f64 * SPEED)
            }

            if forwards_move != 0 {
                let move_forward_direction = state.world.camera.direction;
                state.world.camera.origin = state.world.camera.origin
                    + (move_forward_direction * forwards_move as f64 * SPEED)
            }

            if camera_rotation != 0 {
                state.world.camera = state.world.camera.rotate(camera_rotation as f64 * 0.01);
            }
        });
    }
}
