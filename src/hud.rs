use std::{cell::RefCell, rc::Rc};

use crate::{state::GameState, web};

pub fn setup(state: Rc<RefCell<GameState>>) {
    let document = web::access::document();

    let fps_display_element = document
        .create_element("div")
        .expect("Failed to create fps display div");

    web::access::body().append_child(fps_display_element.as_ref())
        .expect("Failed to append fps display div");

    let mut fps_text = document
        .create_text_node("Testing");
    fps_display_element.append_child(fps_text.as_ref()).expect("Failed to add test to the fps div");

    let time_to_render_display_element = document
        .create_element("div")
        .expect("Failed to create 'time to render' display div");

    web::access::body().append_child(time_to_render_display_element.as_ref())
        .expect("Failed to append 'time to render' display div");

    let mut time_to_render_text = document
        .create_text_node("Testing");
    time_to_render_display_element.append_child(time_to_render_text.as_ref()).expect("Failed to add test to the time_to_render div");

    web::window::run_function_every_animation_frame(move || {
        let state = state.borrow();
        let fps = 1000.0 / state.last_time_between_frames_ms;

        fps_display_element.remove_child(fps_text.as_ref()).expect("Failed to remove previous FPS text");
        fps_text = document
            .create_text_node(format!("FPS: {}", fps.round()).as_str());
        fps_display_element.append_child(fps_text.as_ref()).expect("Failed to add test to the fps div");

        time_to_render_display_element.remove_child(time_to_render_text.as_ref()).expect("Failed to remove previous time_to_render text");
        time_to_render_text = document
            .create_text_node(format!("Time to Render: {}ms", state.last_time_to_render_one_frame_ms.round()).as_str());
        time_to_render_display_element.append_child(time_to_render_text.as_ref()).expect("Failed to add test to the time_to_render div");
    });
}