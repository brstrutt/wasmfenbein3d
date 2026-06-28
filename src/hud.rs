use std::{cell::RefCell, rc::Rc};

use crate::{state::GameState, web};

pub fn setup(state: Rc<RefCell<GameState>>) {
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
