use std::{cell::RefCell, rc::Rc};

use fenbein3d::{render::screen_buffer::ScreenBuffer, state::State};
use web_sys::{Event, EventTarget, MouseEvent};

use crate::wasmutils;

pub fn setup<Screen: ScreenBuffer + 'static>(
    state: Rc<RefCell<State<Screen>>>,
    canvas: web_sys::HtmlCanvasElement,
) {
    setup_mouse_capture_on_click(state.clone(), canvas);
    setup_camera_mouse_control(state);
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
                state.input.sprint = false;
                state.input.move_left = false;
                state.input.move_right = false;
                state.input.move_forward = false;
                state.input.move_backward = false;
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
