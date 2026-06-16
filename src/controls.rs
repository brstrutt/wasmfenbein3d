use std::{cell::{RefCell, RefMut}, rc::Rc};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{Event, KeyboardEvent, MouseEvent, TouchEvent};

use crate::{main_canvas::MainCanvas, state::GameState, web};

pub struct InputState {
    pub move_left: bool,
    pub move_right: bool,
    pub move_forward: bool,
    pub move_backward: bool,
    pub sprint: bool,
    pub camera_rotation: i32,
    pub pointer_locked: bool,
    pub last_canvas_touch_point_x: Option<i32>,
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
            last_canvas_touch_point_x: None,
        }
    }
}

pub fn setup(state: Rc<RefCell<GameState>>, main_canvas: Rc<RefCell<MainCanvas>>) {
    setup_keyboard_movement(state.clone());

    setup_movement_button(state.clone(), "move_forward", |state: &mut RefMut<GameState>, new_value: bool| { state.input.move_forward = new_value; });
    setup_movement_button(state.clone(), "move_backward", |state: &mut RefMut<GameState>, new_value: bool| { state.input.move_backward = new_value; });
    setup_movement_button(state.clone(), "move_left", |state: &mut RefMut<GameState>, new_value: bool| { state.input.move_left = new_value; });
    setup_movement_button(state.clone(), "move_right", |state: &mut RefMut<GameState>, new_value: bool| { state.input.move_right = new_value; });

    setup_mouse_capture_on_click(state.clone(), main_canvas.clone());
    setup_camera_mouse_control(state.clone());

    setup_camera_touch_control(state.clone(), main_canvas.clone());

    setup_character_motion_loop(state.clone());
}

fn setup_movement_button<T: FnMut(&mut RefMut<GameState>, bool) + Clone>(state: Rc<RefCell<GameState>>, button_id: &str, state_change: T) {
    {
        let state = state.clone();
        let mut state_change = state_change.clone();

        let callback = Closure::wrap(Box::new(move |e: Event| {
            e.prevent_default();
            let mut state = state.borrow_mut();
            state_change(&mut state, true);
        }) as Box<dyn FnMut(_)>);
        web::access::button(button_id).set_onmousedown(Some(callback.as_ref().unchecked_ref()));
        web::access::button(button_id).set_ontouchstart(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
    }

    {
        let state = state.clone();
        let mut state_change = state_change.clone();

        let callback = Closure::wrap(Box::new(move |e: Event| {
            e.prevent_default();
            let mut state = state.borrow_mut();
            state_change(&mut state, false);
        }) as Box<dyn FnMut(_)>);
        web::access::button(button_id).set_onmouseup(Some(callback.as_ref().unchecked_ref()));
        web::access::button(button_id).set_ontouchend(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
    }
}

fn setup_keyboard_movement(state: Rc<RefCell<GameState>>) {
    let cloned_state = state.clone();
    web::document::add_event_listener_with_callback("keydown", move |e: KeyboardEvent| {
        let mut state = cloned_state.borrow_mut();
        state.input.sprint = e.shift_key();
        match e.key().as_str() {
            "a" | "A" => state.input.move_left = true,
            "d" | "D" => state.input.move_right = true,
            "w" | "W" => state.input.move_forward = true,
            "s" | "S" => state.input.move_backward = true,
            &_ => return,
        }
    });

    let cloned_state = state.clone();
    web::document::add_event_listener_with_callback("keyup", move |e: KeyboardEvent| {
        let mut state = cloned_state.borrow_mut();
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

fn setup_mouse_capture_on_click(state: Rc<RefCell<GameState>>, main_canvas: Rc<RefCell<MainCanvas>>) {
    let cloned_main_canvas = main_canvas.clone();
    let callback = Closure::wrap(Box::new(move |_e: Event| {
        let main_canvas = cloned_main_canvas.borrow_mut();
        main_canvas.element.request_pointer_lock();
    }) as Box<dyn FnMut(_)>);

    let main_canvas = main_canvas.borrow_mut();
    main_canvas.element.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref()).expect("Failed to setup on click event for canvas");
    callback.forget();

    web::document::add_event_listener_with_callback("pointerlockchange", move |_e: Event| {
        let mut state = state.borrow_mut();
        state.input.pointer_locked = web::access::document().pointer_lock_element().is_some();
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

fn setup_camera_touch_control(state: Rc<RefCell<GameState>>, main_canvas: Rc<RefCell<MainCanvas>>) {
    let main_canvas = main_canvas.borrow_mut();

    let cloned_state = state.clone();
    let touch_start_closure = Closure::wrap(Box::new(move |e: TouchEvent| {
        e.prevent_default();
        let mut state = cloned_state.borrow_mut();

        let touch_points = e.target_touches();
        if touch_points.length() > 0 {
            let touch_x_position = touch_points.item(0).expect("Failed to get first touch point on the canvas").screen_x();
            state.input.last_canvas_touch_point_x = Some(touch_x_position);

        }
    }) as Box<dyn FnMut(_)>);
    main_canvas.element.set_ontouchstart(Some(touch_start_closure.as_ref().unchecked_ref()));
    touch_start_closure.forget();


    let cloned_state = state.clone();
    let touch_move_closure = Closure::wrap(Box::new(move |e: TouchEvent| {
        e.prevent_default();
        let mut state = cloned_state.borrow_mut();
        const ACCELERATION: i32 = 4;

        let touch_points = e.target_touches();
        if touch_points.length() > 0 {
            let touch_x_position = touch_points.item(0).expect("Failed to get first touch point on the canvas").screen_x();

            if state.input.last_canvas_touch_point_x.is_some() {
                state.input.camera_rotation = (state.input.last_canvas_touch_point_x.unwrap() - touch_x_position) * ACCELERATION;
            }

            state.input.last_canvas_touch_point_x = Some(touch_x_position);
        }
    }) as Box<dyn FnMut(_)>);
    main_canvas.element.set_ontouchmove(Some(touch_move_closure.as_ref().unchecked_ref()));
    touch_move_closure.forget();


    let cloned_state = state.clone();
    let touch_end_closure = Closure::wrap(Box::new(move |e: TouchEvent| {
        e.prevent_default();
        let mut state = cloned_state.borrow_mut();
        state.input.last_canvas_touch_point_x = None;
    }) as Box<dyn FnMut(_)>);
    main_canvas.element.set_ontouchend(Some(touch_end_closure.as_ref().unchecked_ref()));
    touch_end_closure.forget();
}

fn setup_character_motion_loop(state: Rc<RefCell<GameState>>) {
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
