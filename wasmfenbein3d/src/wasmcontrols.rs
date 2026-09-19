use std::{cell::RefCell, rc::Rc};

use fenbein3d::{render::screen_buffer::ScreenBuffer, state::State};

mod phsyics_loop;
mod screen_controls;

pub mod data_action {
    pub const MOVE_LEFT: &str = "wasmfenbein3d_character_input_move_left";
    pub const MOVE_RIGHT: &str = "wasmfenbein3d_character_input_move_right";
    pub const MOVE_FORWARD: &str = "wasmfenbein3d_character_input_move_forward";
    pub const MOVE_BACK: &str = "wasmfenbein3d_character_input_move_back";
}

pub fn setup<Screen: ScreenBuffer + 'static>(state: Rc<RefCell<State<Screen>>>) {
    screen_controls::setup(state.clone());
    phsyics_loop::setup_character_motion_loop(state);
}
