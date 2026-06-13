use std::{cell::RefCell, rc::Rc};
use web_sys::{KeyboardEvent};

use crate::{state::GameState, web};

pub struct InputState {
    pub moveLeft: bool,
    pub moveRight: bool,
    pub moveForward: bool,
    pub moveBackward: bool,
}

impl InputState {
    pub fn setup() -> InputState {
        InputState { moveLeft: false, moveRight: false, moveForward: false, moveBackward: false }
    }
}

pub fn setup(state: Rc<RefCell<GameState>>) {
    web::document::add_event_listener_with_callback(move |e: KeyboardEvent| {
        let mut state = state.borrow_mut();
        match e.key().as_str() {
            "a" | "A" => state.world.camera.origin = state.world.camera.origin - state.world.camera.rotate(std::f32::consts::PI as f64 / 2.0).direction,
            "d" | "D" => state.world.camera.origin = state.world.camera.origin + state.world.camera.rotate(std::f32::consts::PI as f64 / 2.0).direction,
            "w" | "W" => state.world.camera.origin = state.world.camera.origin + state.world.camera.direction,
            "s" | "S" => state.world.camera.origin = state.world.camera.origin - state.world.camera.direction,
            "ArrowRight" => state.world.camera = state.world.camera.rotate(0.1),
            "ArrowLeft" => state.world.camera = state.world.camera.rotate(-0.1),
            &_ => return,
        }
    });
}