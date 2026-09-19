use std::{cell::RefCell, rc::Rc};

use fenbein3d::{
    controls::{Direction, MovementEvent},
    motion,
    render::screen_buffer::ScreenBuffer,
    state::State,
};
use web_sys::{Element, Event, EventTarget, wasm_bindgen::JsCast};

use crate::wasmutils;

pub mod data_action {
    pub const MOVE_LEFT: &str = "wasmfenbein3d_character_input_move_left";
    pub const MOVE_RIGHT: &str = "wasmfenbein3d_character_input_move_right";
    pub const MOVE_FORWARD: &str = "wasmfenbein3d_character_input_move_forward";
    pub const MOVE_BACK: &str = "wasmfenbein3d_character_input_move_back";
}

pub fn setup<Screen: ScreenBuffer + 'static>(state: Rc<RefCell<State<Screen>>>) {
    setup_movement_button(state.clone(), data_action::MOVE_FORWARD, Direction::Forward);
    setup_movement_button(state.clone(), data_action::MOVE_BACK, Direction::Backward);
    setup_movement_button(state.clone(), data_action::MOVE_LEFT, Direction::Left);
    setup_movement_button(state.clone(), data_action::MOVE_RIGHT, Direction::Right);

    setup_character_motion_loop(state);
}

fn setup_movement_button<Screen: ScreenBuffer + 'static>(
    state: Rc<RefCell<State<Screen>>>,
    input_element_data_action: &'static str,
    direction: Direction,
) {
    {
        let state = state.clone();
        let direction = direction.clone();
        wasmutils::add_event_listener_with_callback(
            &mut EventTarget::from(wasmutils::document()),
            "pointerdown",
            move |event: Event| {
                if does_event_target_have_data_action(event.target(), input_element_data_action) {
                    state
                        .borrow_mut()
                        .input
                        .change_direction(&direction, MovementEvent::Start);
                }
            },
        );
    }
    {
        let state = state.clone();
        let direction = direction.clone();
        wasmutils::add_event_listener_with_callback(
            &mut EventTarget::from(wasmutils::document()),
            "pointerup",
            move |event: Event| {
                if does_event_target_have_data_action(event.target(), input_element_data_action) {
                    state
                        .borrow_mut()
                        .input
                        .change_direction(&direction, MovementEvent::Stop);
                }
            },
        );
    }
}

fn does_event_target_have_data_action(
    event_target: Option<EventTarget>,
    data_action: &'static str,
) -> bool {
    if let Some(target) = event_target {
        if let Ok(element) = target.dyn_into::<Element>() {
            if let Some(element_data_action) = element.get_attribute("data-action") {
                return element_data_action == data_action;
            }
        }
    }
    false
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
