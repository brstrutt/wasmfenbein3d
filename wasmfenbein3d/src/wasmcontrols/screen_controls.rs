use std::{cell::RefCell, rc::Rc};

use fenbein3d::{
    controls::{Direction, MovementEvent},
    render::screen_buffer::ScreenBuffer,
    state::State,
};
use web_sys::{Element, Event, EventTarget, wasm_bindgen::JsCast};

use super::data_action;
use crate::wasmutils;

pub fn setup<Screen: ScreenBuffer + 'static>(state: Rc<RefCell<State<Screen>>>) {
    setup_movement_button(state.clone(), data_action::MOVE_FORWARD, Direction::Forward);
    setup_movement_button(state.clone(), data_action::MOVE_BACK, Direction::Backward);
    setup_movement_button(state.clone(), data_action::MOVE_LEFT, Direction::Left);
    setup_movement_button(state.clone(), data_action::MOVE_RIGHT, Direction::Right);
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
