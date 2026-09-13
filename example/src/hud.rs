use std::{cell::RefCell, rc::Rc};

use wasmfenbein3d::{render::screen_buffer::ScreenBuffer, state::State};

use crate::web;

pub fn setup<Screen: ScreenBuffer + 'static>(state: Rc<RefCell<State<Screen>>>) {
    setup_fps_display(state.clone());
    setup_display_resolution(state);
}

fn setup_fps_display<Screen: ScreenBuffer + 'static>(state: Rc<RefCell<State<Screen>>>) {
    let document = web::access::document();

    let fps_display_element = document
        .get_element_by_id("fps_display")
        .expect("Failed to get fps display div");

    let time_to_render_display_element = document
        .get_element_by_id("time_to_render_display")
        .expect("Failed to get 'time to render' display div");

    web::window::run_function_every_animation_frame(move || {
        let state = state.borrow();
        let fps = 1000.0 / state.stats.render_frame.last_duration_ms;

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
                    state.stats.render_frame.last_duration_ms.round()
                )
                .as_str(),
            ));
    });
}

fn setup_display_resolution<Screen: ScreenBuffer>(state: Rc<RefCell<State<Screen>>>) {
    let screen = &state.borrow().screen_buffer;
    let document = web::access::document();

    let resolution_display_element = document
        .get_element_by_id("resolution_display")
        .expect("Failed to get resolution display div");

    resolution_display_element
        .first_child()
        .expect("Couldnt get Resolution display child")
        .set_text_content(Some(
            format!("Render resolution: {}x{}", screen.width(), screen.height()).as_str(),
        ));
}
