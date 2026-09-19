use crate::wasmutils;
use fenbein3d::{motion, render::screen_buffer::ScreenBuffer, state::State};
use std::{cell::RefCell, rc::Rc};

pub fn setup<Screen: ScreenBuffer + 'static>(state: Rc<RefCell<State<Screen>>>) {
    setup_character_motion_loop(state.clone());
    setup_camera_motion_loop(state);
}

fn setup_character_motion_loop<Screen: ScreenBuffer + 'static>(state: Rc<RefCell<State<Screen>>>) {
    wasmutils::run_function_every_animation_frame(move || {
        let mut state = state.borrow_mut();
        let current_time = wasmutils::now_in_ms();
        state.stats.physics_frame.last_duration_ms =
            current_time - state.stats.physics_frame.last_time_ms;
        state.stats.physics_frame.last_time_ms = current_time;

        let time_since_last_frame_s = state.stats.physics_frame.last_duration_ms / 1000.0;

        let velocity_per_s = if state.input.sprint { 12.0 } else { 4.0 };
        let velocity = velocity_per_s * time_since_last_frame_s;

        let camera_rotation = state.camera.ray.get_angle();
        let motion = state
            .input
            .get_cameraspace_movement_direction()
            .rotate(camera_rotation)
            * velocity;

        state.camera.ray.origin =
            motion::move_object(state.camera.ray.origin, &motion, &state.world);
        state.camera.refresh_screen_rays();
    });
}

fn setup_camera_motion_loop<Screen: ScreenBuffer + 'static>(state: Rc<RefCell<State<Screen>>>) {
    wasmutils::run_function_every_animation_frame(move || {
        let mut state = state.borrow_mut();

        const ROTATION_SPEED: f64 = 0.001;

        let camera_rotation = state.input.camera_rotation;
        state.input.camera_rotation = 0;

        if camera_rotation != 0 {
            state.camera = state.camera.rotate(camera_rotation as f64 * ROTATION_SPEED);
            state.camera.refresh_screen_rays();
        }
    });
}
