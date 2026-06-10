mod rgb;
mod screen;
mod primitives;
mod world;
mod camera;

use std::{cell::RefCell, rc::Rc};

use screen::Screen;
use rgb::RGB;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

use crate::{camera::Camera, world::World};

fn main() {
    console_error_panic_hook::set_once();

    log("Starting up!");
    setup_render_loop();
}

fn render(screen: &Screen, world: &World, camera: &Camera, i: &mut u32) {
    let timer_label = format!("Render run {}", i);
    web_sys::console::time_with_label(&timer_label);
    screen.clear();


    for x in 0..=screen.width {
        let ray = camera.ray_for_column(x);
        let wall_distance = world.dist_to_wall(&ray);

        if wall_distance.is_some() {
            let distance = wall_distance.unwrap().round() as u32;
            let height = screen.height * 10 / (distance + 100);
            screen.render_column(x, height, &RGB {red: 30 * 100 / distance, green: 150 * 100 / distance, blue: 30 * 100 / distance});
        }
    }


    web_sys::console::time_end_with_label(&timer_label);

    *i = (*i + 1) % 500;
}

fn setup_render_loop() {
    let screen = Screen::init();
    let world = World::dummy();
    let camera = Camera::dummy();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut i: u32 = 0;

    *g.borrow_mut() = Some(Closure::new(move || {
        // do the animation code here
        render(&screen, &world, &camera, &mut i);
        // queue up another re-draw request
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    // queue up the first re-draw request, to start animation
    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn log(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}
