use std::{cell::RefCell, rc::Rc};

use wasmfenbein3d::core::{render::screen::ScreenBuffer, state::GameState};

use crate::web;

pub fn setup(state: Rc<RefCell<GameState>>, screen: Rc<RefCell<ScreenBuffer>>) {
    setup_fps_tracking(state.clone());
    setup_fps_display(state);
    setup_display_resolution(screen);
}

fn setup_fps_display(state: Rc<RefCell<GameState>>) {
    let document = web::access::document();

    let fps_display_element = document
        .get_element_by_id("fps_display")
        .expect("Failed to get fps display div");

    let time_to_render_display_element = document
        .get_element_by_id("time_to_render_display")
        .expect("Failed to get 'time to render' display div");

    web::window::run_function_every_animation_frame(move || {
        let state = state.borrow();
        let fps = 1000.0 / state.last_time_between_frames_ms;

        fps_display_element
            .first_child()
            .expect("Couldnt get FPS display child")
            .set_text_content(Some(format!("FPS: {}", fps.round()).as_str()));
        time_to_render_display_element
            .first_child()
            .expect("Couldnt get FPS display child")
            .set_text_content(Some(
                format!(
                    "Time to Render: {}ms",
                    state.last_time_to_render_one_frame_ms.round()
                )
                .as_str(),
            ));
    });
}

fn setup_fps_tracking(state: Rc<RefCell<GameState>>) {
    web::window::run_function_every_animation_frame(move || {
        let mut state = state.borrow_mut();
        let current_time = web::window::now_in_ms();
        let time_since_last_frame_ms = current_time - state.last_frame_time_ms;
        state.last_frame_time_ms = current_time;
        state.last_time_between_frames_ms = time_since_last_frame_ms;
    });
}

fn setup_display_resolution(screen: Rc<RefCell<ScreenBuffer>>) {
    let screen = screen.borrow();
    let document = web::access::document();

    let resolution_display_element = document
        .get_element_by_id("resolution_display")
        .expect("Failed to get resolution display div");

    resolution_display_element
        .first_child()
        .expect("Couldnt get Resolution display child")
        .set_text_content(Some(
            format!("Render resolution: {}x{}", screen.width, screen.height).as_str(),
        ));
}
