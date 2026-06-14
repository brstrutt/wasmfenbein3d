use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{Event, KeyboardEvent, MouseEvent};

use crate::{main_canvas::MainCanvas, state::GameState, web};

pub struct InputState {
    pub move_left: bool,
    pub move_right: bool,
    pub move_forward: bool,
    pub move_backward: bool,
    pub sprint: bool,
    pub camera_rotation: i32,
    pub pointer_locked: bool,
}

impl InputState {
    pub fn setup() -> InputState {
        InputState {
            move_left: false,
            move_right: false,
            move_forward: false,
            move_backward: false,
            sprint: false,
            camera_rotation: 0,
            pointer_locked: false,
        }
    }
}

pub fn setup(state: Rc<RefCell<GameState>>, main_canvas: Rc<RefCell<MainCanvas>>) {
    {
        let state = state.clone();
        web::document::add_event_listener_with_callback("keydown", move |e: KeyboardEvent| {
            let mut state = state.borrow_mut();
            state.input.sprint = e.shift_key();
            match e.key().as_str() {
                "a" | "A" => state.input.move_left = true,
                "d" | "D" => state.input.move_right = true,
                "w" | "W" => state.input.move_forward = true,
                "s" | "S" => state.input.move_backward = true,
                &_ => return,
            }
        });
    }

    {
        let state = state.clone();
        web::document::add_event_listener_with_callback("keyup", move |e: KeyboardEvent| {
            let mut state = state.borrow_mut();
            state.input.sprint = e.shift_key();
            match e.key().as_str() {
                "a" | "A" => state.input.move_left = false,
                "d" | "D" => state.input.move_right = false,
                "w" | "W" => state.input.move_forward = false,
                "s" | "S" => state.input.move_backward = false,
                &_ => return,
            }
        });
    }

    {
        let state = state.clone();
        web::window::run_function_every_animation_frame(move || {
            let mut state = state.borrow_mut();
            let current_time = web::window::now_in_ms();
            let time_since_last_frame_ms = current_time - state.last_frame_time_ms;
            state.last_frame_time_ms = current_time;
            state.last_time_between_frames_ms = time_since_last_frame_ms;

            let time_since_last_frame_s = time_since_last_frame_ms / 1000.0;

            const MOVEMENT_SPEED: f64 = 2.0; // move 2.0 per second
            const ROTATION_SPEED: f64 = 0.05; // roate 0.05 per second

            let sprint_speed = if state.input.sprint {5.0} else {1.0};

            let mut sideways_move = 0;
            if state.input.move_left {
                sideways_move -= 1;
            }
            if state.input.move_right {
                sideways_move += 1;
            }

            let mut forwards_move = 0;
            if state.input.move_forward {
                forwards_move += 1;
            }
            if state.input.move_backward {
                forwards_move -= 1;
            }

            let camera_rotation = state.input.camera_rotation;
            state.input.camera_rotation = 0;

            if sideways_move != 0 {
                let move_right_direction = state
                    .world
                    .camera
                    .rotate(std::f32::consts::PI as f64 / 2.0)
                    .direction;
                state.world.camera.origin = state.world.camera.origin
                    + (move_right_direction * sideways_move as f64 * MOVEMENT_SPEED * sprint_speed * time_since_last_frame_s)
            }

            if forwards_move != 0 {
                let move_forward_direction = state.world.camera.direction;
                state.world.camera.origin = state.world.camera.origin
                    + (move_forward_direction * forwards_move as f64 * MOVEMENT_SPEED * sprint_speed * time_since_last_frame_s)
            }

            if camera_rotation != 0 {
                state.world.camera = state.world.camera.rotate(camera_rotation as f64 * ROTATION_SPEED * time_since_last_frame_s);
            }
        });
    }


    {
        let main_canvas_inner = main_canvas.clone();
        let main_canvas_outer = main_canvas.clone();

        let callback = Closure::wrap(Box::new(move |_e: Event| {
            let main_canvas = main_canvas_inner.borrow_mut();
            main_canvas.element.request_pointer_lock();
        }) as Box<dyn FnMut(_)>);

        let main_canvas = main_canvas_outer.borrow_mut();
        main_canvas.element.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref()).expect("Failed to setup on click event for canvas");
        callback.forget();


        let state = state.clone();
        web::document::add_event_listener_with_callback("pointerlockchange", move |_e: Event| {
            let mut state = state.borrow_mut();
            state.input.pointer_locked = web::access::document().pointer_lock_element().is_some();
        });
    }

    {
        let state = state.clone();
        web::document::add_event_listener_with_callback("mousemove", move |e: MouseEvent| {
            let mut state = state.borrow_mut();

            if state.input.pointer_locked {
                state.input.camera_rotation += e.movement_x();
            }
        });
    }
}
