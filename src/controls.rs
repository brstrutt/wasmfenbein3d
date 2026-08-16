use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use wasm_bindgen::{JsCast, prelude::Closure};
use wasmfenbein3d::core::{motion, state::GameState};
use web_sys::{Event, KeyboardEvent, MouseEvent, TouchEvent};

use crate::web;

pub fn setup(state: Rc<RefCell<GameState>>) {
    setup_keyboard_movement(state.clone());

    setup_movement_button(
        state.clone(),
        "move_forward",
        |state: &mut RefMut<GameState>, new_value: bool| {
            state.input.move_forward = new_value;
        },
    );
    setup_movement_button(
        state.clone(),
        "move_backward",
        |state: &mut RefMut<GameState>, new_value: bool| {
            state.input.move_backward = new_value;
        },
    );
    setup_movement_button(
        state.clone(),
        "move_left",
        |state: &mut RefMut<GameState>, new_value: bool| {
            state.input.move_left = new_value;
        },
    );
    setup_movement_button(
        state.clone(),
        "move_right",
        |state: &mut RefMut<GameState>, new_value: bool| {
            state.input.move_right = new_value;
        },
    );

    setup_mouse_capture_on_click(state.clone());
    setup_camera_mouse_control(state.clone());
    setup_click_passthrough(state.clone());

    setup_camera_touch_control(state.clone());

    setup_character_motion_loop(state.clone());
    setup_camera_motion_loop(state.clone());
}

fn setup_movement_button<T: FnMut(&mut RefMut<GameState>, bool) + Clone>(
    state: Rc<RefCell<GameState>>,
    button_id: &str,
    state_change: T,
) {
    let button = web::access::button(button_id);

    {
        let state = state.clone();
        let mut state_change = state_change.clone();

        let button_clone = button.clone();
        let callback = Closure::wrap(Box::new(move |e: Event| {
            e.prevent_default();
            button_clone.set_class_name("active");

            let mut state = state.borrow_mut();
            state_change(&mut state, true);
        }) as Box<dyn FnMut(_)>);
        button.set_onmousedown(Some(callback.as_ref().unchecked_ref()));
        button.set_ontouchstart(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
    }

    {
        let state = state.clone();
        let mut state_change = state_change.clone();

        let button_clone = button.clone();
        let callback = Closure::wrap(Box::new(move |e: Event| {
            e.prevent_default();
            button_clone.set_class_name("");

            let mut state = state.borrow_mut();
            state_change(&mut state, false);
        }) as Box<dyn FnMut(_)>);
        button.set_onmouseup(Some(callback.as_ref().unchecked_ref()));
        button.set_ontouchend(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
    }
}

fn setup_keyboard_movement(state: Rc<RefCell<GameState>>) {
    let cloned_state = state.clone();
    web::document::add_event_listener_with_callback("keydown", move |e: KeyboardEvent| {
        let mut state = cloned_state.borrow_mut();
        if state.input.pointer_locked {
            state.input.sprint = e.shift_key();
            match e.key().as_str() {
                "a" | "A" => state.input.move_left = true,
                "d" | "D" => state.input.move_right = true,
                "w" | "W" => state.input.move_forward = true,
                "s" | "S" => state.input.move_backward = true,
                &_ => return,
            }
        }
    });

    let cloned_state = state.clone();
    web::document::add_event_listener_with_callback("keyup", move |e: KeyboardEvent| {
        let mut state = cloned_state.borrow_mut();
        if state.input.pointer_locked {
            state.input.sprint = e.shift_key();
            match e.key().as_str() {
                "a" | "A" => state.input.move_left = false,
                "d" | "D" => state.input.move_right = false,
                "w" | "W" => state.input.move_forward = false,
                "s" | "S" => state.input.move_backward = false,
                &_ => return,
            }
        }
    });
}

fn setup_mouse_capture_on_click(state: Rc<RefCell<GameState>>) {
    web::main_canvas::add_event_listener_with_callback("click", move |_e: Event| {
        web::access::main_canvas().request_pointer_lock();
    });
    web::document::add_event_listener_with_callback("pointerlockchange", move |_e: Event| {
        let mut state = state.borrow_mut();
        state.input.pointer_locked = web::access::document().pointer_lock_element().is_some();
        if !state.input.pointer_locked {
            state.input.sprint = false;
            state.input.move_left = false;
            state.input.move_right = false;
            state.input.move_forward = false;
            state.input.move_backward = false;
        }
    });
}

fn setup_click_passthrough(state: Rc<RefCell<GameState>>) {
    let cloned_state = state.clone();
    web::main_canvas::add_event_listener_with_callback("click", move |_e: MouseEvent| {
        let state = cloned_state.borrow();
        state.input.trigger_click(&state.world);
    });

    let cloned_state = state.clone();
    web::main_canvas::add_event_listener_with_callback("touchmove", move |_e: TouchEvent| {
        let mut state = cloned_state.borrow_mut();

        state.input.touch_has_moved_camera = true;
    });

    web::main_canvas::add_event_listener_with_callback("touchend", move |_e: TouchEvent| {
        let mut state = state.borrow_mut();

        if !state.input.touch_has_moved_camera {
            state.input.trigger_click(&state.world);
        }
        state.input.touch_has_moved_camera = false;
    });
}

fn setup_camera_mouse_control(state: Rc<RefCell<GameState>>) {
    web::document::add_event_listener_with_callback("mousemove", move |e: MouseEvent| {
        let mut state = state.borrow_mut();

        if state.input.pointer_locked {
            state.input.camera_rotation += e.movement_x();
        }
    });
}

fn setup_camera_touch_control(state: Rc<RefCell<GameState>>) {
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

fn setup_character_motion_loop(state: Rc<RefCell<GameState>>) {
    web::window::run_function_every_animation_frame(move || {
        let mut state = state.borrow_mut();
        let current_time = web::window::now_in_ms();
        let time_since_last_frame_ms = current_time - state.last_frame_time_ms;

        let time_since_last_frame_s = time_since_last_frame_ms / 1000.0;

        let velocity_per_s = if state.input.sprint { 12.0 } else { 4.0 };
        let velocity = velocity_per_s * time_since_last_frame_s;

        let camera_rotation = state.world.camera.ray.get_angle();
        let motion = state
            .input
            .get_cameraspace_movement_direction()
            .rotate(camera_rotation)
            * velocity;

        state.world.camera.ray.origin =
            motion::move_object(state.world.camera.ray.origin, &motion, &state.world);
        state.world.camera.refresh_screen_rays();
    });
}

fn setup_camera_motion_loop(state: Rc<RefCell<GameState>>) {
    web::window::run_function_every_animation_frame(move || {
        let mut state = state.borrow_mut();

        const ROTATION_SPEED: f64 = 0.001;

        let camera_rotation = state.input.camera_rotation;
        state.input.camera_rotation = 0;

        if camera_rotation != 0 {
            state.world.camera = state
                .world
                .camera
                .rotate(camera_rotation as f64 * ROTATION_SPEED);
            state.world.camera.refresh_screen_rays();
        }
    });
}
