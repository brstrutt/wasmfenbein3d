use std::{cell::RefCell, rc::Rc};

use fenbein3d::{
    controls::{Direction, MovementEvent},
    render::screen_buffer::ScreenBuffer,
    state::State,
};
use web_sys::{Element, Event, EventTarget, TouchEvent, wasm_bindgen::JsCast};

use super::data_action;
use crate::wasmutils;

pub fn setup<Screen: ScreenBuffer + 'static>(
    state: Rc<RefCell<State<Screen>>>,
    canvas: web_sys::HtmlCanvasElement,
) {
    setup_movement_button(state.clone(), data_action::MOVE_FORWARD, Direction::Forward);
    setup_movement_button(state.clone(), data_action::MOVE_BACK, Direction::Backward);
    setup_movement_button(state.clone(), data_action::MOVE_LEFT, Direction::Left);
    setup_movement_button(state.clone(), data_action::MOVE_RIGHT, Direction::Right);
    setup_camera_touch_control(state, canvas);
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

fn setup_camera_touch_control<Screen: ScreenBuffer + 'static>(
    state: Rc<RefCell<State<Screen>>>,
    canvas: web_sys::HtmlCanvasElement,
) {
    let cloned_state = state.clone();
    wasmutils::add_event_listener_with_callback(
        &mut EventTarget::from(canvas.clone()),
        "touchstart",
        move |e: TouchEvent| {
            e.prevent_default();
            let mut state = cloned_state.borrow_mut();

            let touch_points = e.target_touches();
            if touch_points.length() > 0 {
                let touch_x_position = touch_points
                    .item(0)
                    .expect("Failed to get first touch point on the canvas")
                    .screen_x();
                state.input.last_canvas_touch_point_x = Some(touch_x_position);
            }
        },
    );

    let cloned_state = state.clone();
    wasmutils::add_event_listener_with_callback(
        &mut EventTarget::from(canvas.clone()),
        "touchmove",
        move |e: TouchEvent| {
            e.prevent_default();
            let mut state = cloned_state.borrow_mut();
            const ACCELERATION: i32 = 4;

            let touch_points = e.target_touches();
            if touch_points.length() > 0 {
                let touch_x_position = touch_points
                    .item(0)
                    .expect("Failed to get first touch point on the canvas")
                    .screen_x();

                if state.input.last_canvas_touch_point_x.is_some() {
                    state.input.camera_rotation = (state.input.last_canvas_touch_point_x.unwrap()
                        - touch_x_position)
                        * ACCELERATION;
                }

                state.input.last_canvas_touch_point_x = Some(touch_x_position);
                state.input.touch_has_moved_camera = true;
            }
        },
    );

    let cloned_state = state.clone();
    wasmutils::add_event_listener_with_callback(
        &mut EventTarget::from(canvas),
        "touchend",
        move |e: TouchEvent| {
            e.prevent_default();
            let mut state = cloned_state.borrow_mut();
            state.input.last_canvas_touch_point_x = None;
        },
    );
}
