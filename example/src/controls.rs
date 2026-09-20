use std::{cell::RefCell, rc::Rc};

use wasmfenbein3d::{render::screen_buffer::ScreenBuffer, state::State};
use web_sys::{MouseEvent, TouchEvent};

use crate::{textures, web};

pub fn setup<Screen: ScreenBuffer + 'static>(state: Rc<RefCell<State<Screen>>>) {
    setup_click_passthrough(state.clone());
    setup_camera_touch_control(state.clone());
}

fn setup_click_passthrough<Screen: ScreenBuffer + 'static>(state: Rc<RefCell<State<Screen>>>) {
    let cloned_state = state.clone();
    web::main_canvas::add_event_listener_with_callback("click", move |_e: MouseEvent| {
        let state = cloned_state.borrow();
        let item_ids = state.input.get_items_under_cursor(&state);
        for id in item_ids {
            on_click(id);
        }
    });

    let cloned_state = state.clone();
    web::main_canvas::add_event_listener_with_callback("touchmove", move |_e: TouchEvent| {
        let mut state = cloned_state.borrow_mut();

        state.input.touch_has_moved_camera = true;
    });

    web::main_canvas::add_event_listener_with_callback("touchend", move |_e: TouchEvent| {
        let mut state = state.borrow_mut();

        if !state.input.touch_has_moved_camera {
            let item_ids = state.input.get_items_under_cursor(&state);
            for id in item_ids {
                on_click(id);
            }
        }
        state.input.touch_has_moved_camera = false;
    });
}

fn on_click(item_id: String) {
    match item_id.as_str() {
        val if val == textures::NOKIA_ART_JAM_3_HOUSE.id => {
            log::info!("House is thinking it's not Lupus!");
        }
        val if val == textures::NOKIA_ART_JAM_3_KEYBOARD_CAT.id => {
            log::info!("Look at that cat GO!");
        }
        val if val == textures::NOKIA_ART_JAM_3_WORMS.id => {
            log::info!("Damn these worms are ANGRY!");
        }
        val if val == textures::VERMINTIDE_TAPESTRY.id => {
            log::info!("Clicked on the tapestry!");
        }
        val if val == textures::UBERSREIK_FIVE.id => {
            let popup_page = web::access::popup_page();
            if popup_page.hidden() {
                web::access::document().exit_pointer_lock();
            }
            popup_page.set_hidden(!popup_page.hidden());
        }
        &_ => {}
    }
}

fn setup_camera_touch_control<Screen: ScreenBuffer + 'static>(state: Rc<RefCell<State<Screen>>>) {
    let cloned_state = state.clone();
    web::main_canvas::add_event_listener_with_callback("touchstart", move |e: TouchEvent| {
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
    });

    let cloned_state = state.clone();
    web::main_canvas::add_event_listener_with_callback("touchmove", move |e: TouchEvent| {
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
    });

    let cloned_state = state.clone();
    web::main_canvas::add_event_listener_with_callback("touchend", move |e: TouchEvent| {
        e.prevent_default();
        let mut state = cloned_state.borrow_mut();
        state.input.last_canvas_touch_point_x = None;
    });
}
