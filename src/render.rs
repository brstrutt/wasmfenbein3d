use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, prelude::Closure};
use crate::{main_canvas::MainCanvas, render::rgb::RGB, world::World};


pub mod screen;
pub mod rgb;

pub fn setup(world: Rc<RefCell<World>>, canvas: MainCanvas) {
    run_function_on_animation_frame(move || {
        render(&canvas, &world);
    });
}

fn run_function_on_animation_frame<T: FnMut()>(mut run: T) where T: 'static{
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        // do the animation code here
        run();
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


pub fn render(canvas: &MainCanvas, world: &RefCell<World>) {
    screen::clear(canvas);
    let world = world.borrow();

    for x in 0..=canvas.element.width() {
        let ray = world.camera.ray_for_column(x);
        let wall_distance = world.dist_to_wall(&ray);

        if wall_distance.is_some() {
            let distance = wall_distance.unwrap().round() as u32;
            let height = canvas.element.height() * 10 / (distance + 100);
            screen::render_column(canvas, x, height, &RGB {red: 30 * 100 / distance, green: 150 * 100 / distance, blue: 30 * 100 / distance});
        }
    }
}
