use std::{cell::RefCell, rc::Rc};

use fenbein3d::{controls::MovementEvent, render::screen_buffer::ScreenBuffer, state::State};
use web_sys::{Event, EventTarget, KeyboardEvent, MouseEvent};

use super::utils;
use crate::wasmutils;

pub fn setup<Screen: ScreenBuffer + 'static>(
    state: Rc<RefCell<State<Screen>>>,
    canvas: web_sys::HtmlCanvasElement,
) {
    setup_mouse_capture_on_click(state.clone(), canvas);
    setup_camera_mouse_control(state.clone());
    setup_keyboard_movement_controls(state);
}

fn setup_mouse_capture_on_click<Screen: ScreenBuffer + 'static>(
    state: Rc<RefCell<State<Screen>>>,
    canvas: web_sys::HtmlCanvasElement,
) {
    wasmutils::add_event_listener_with_callback(
        &mut EventTarget::from(canvas.clone()),
        "click",
        move |_e: Event| {
            canvas.request_pointer_lock();
        },
    );
    wasmutils::add_event_listener_with_callback(
        &mut EventTarget::from(wasmutils::document()),
        "pointerlockchange",
        move |_e: Event| {
            let mut state = state.borrow_mut();
            state.input.pointer_locked = wasmutils::document().pointer_lock_element().is_some();
            if !state.input.pointer_locked {
                state.input.reset_movement();
            }
        },
    );
}

fn setup_camera_mouse_control<Screen: ScreenBuffer + 'static>(state: Rc<RefCell<State<Screen>>>) {
    wasmutils::add_event_listener_with_callback(
        &mut EventTarget::from(wasmutils::document()),
        "mousemove",
        move |e: MouseEvent| {
            let mut state = state.borrow_mut();

            if state.input.pointer_locked {
                state.input.camera_rotation += e.movement_x();
            }
        },
    );
}

fn setup_keyboard_movement_controls<Screen: ScreenBuffer + 'static>(
    state: Rc<RefCell<State<Screen>>>,
) {
    let cloned_state = state.clone();
    wasmutils::add_event_listener_with_callback(
        &mut EventTarget::from(wasmutils::document()),
        "keydown",
        move |e: KeyboardEvent| {
            let mut state = cloned_state.borrow_mut();
            if state.input.pointer_locked {
                state.input.sprint = e.shift_key();
                if let Some(direction) = utils::key_to_direction(e.key().as_str()) {
                    state
                        .input
                        .change_direction(&direction, MovementEvent::Start);
                }
            }
        },
    );

    let cloned_state = state.clone();
    wasmutils::add_event_listener_with_callback(
        &mut EventTarget::from(wasmutils::document()),
        "keyup",
        move |e: KeyboardEvent| {
            let mut state = cloned_state.borrow_mut();
            if state.input.pointer_locked {
                state.input.sprint = e.shift_key();
                if let Some(direction) = utils::key_to_direction(e.key().as_str()) {
                    state
                        .input
                        .change_direction(&direction, MovementEvent::Stop);
                }
            }
        },
    );
}
