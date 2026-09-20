use std::{cell::RefCell, rc::Rc};

use fenbein3d::{render::screen_buffer::ScreenBuffer, state::State};
use web_sys::{EventTarget, HtmlCanvasElement, MouseEvent, TouchEvent};

use crate::wasmutils;

pub fn setup<Screen: ScreenBuffer + 'static>(
    state: Rc<RefCell<State<Screen>>>,
    canvas: HtmlCanvasElement,
    on_click: impl Fn(&str) + Clone,
) {
    {
        let state = state.clone();
        let on_click = on_click.clone();
        wasmutils::add_event_listener_with_callback(
            &mut EventTarget::from(canvas.clone()),
            "click",
            move |_e: MouseEvent| {
                let state = state.borrow();

                if state.input.pointer_locked {
                    let item_ids = state.input.get_items_under_cursor(&state);
                    for id in item_ids {
                        on_click(id.as_str());
                    }
                }
            },
        );
    }

    {
        let state = state.clone();
        wasmutils::add_event_listener_with_callback(
            &mut EventTarget::from(canvas.clone()),
            "touchmove",
            move |_e: TouchEvent| {
                let mut state = state.borrow_mut();

                state.input.touch_has_moved_camera = true;
            },
        );
    }

    {
        let on_click = on_click.clone();
        wasmutils::add_event_listener_with_callback(
            &mut EventTarget::from(canvas),
            "touchend",
            move |_e: TouchEvent| {
                let mut state = state.borrow_mut();

                if !state.input.touch_has_moved_camera {
                    if state.input.pointer_locked {
                        let item_ids = state.input.get_items_under_cursor(&state);
                        for id in item_ids {
                            on_click(id.as_str());
                        }
                        state.input.pointer_locked = false;
                    } else {
                        state.input.pointer_locked = true;
                    }
                }
                state.input.touch_has_moved_camera = false;
            },
        );
    }
}
