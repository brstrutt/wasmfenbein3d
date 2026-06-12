use std::{cell::RefCell, rc::Rc};
use crate::{main_canvas::MainCanvas, render::rgb::RGB, web, world::World};

pub mod screen;
pub mod rgb;

pub fn setup(world: Rc<RefCell<World>>, canvas: MainCanvas) {
    web::window::run_function_every_animation_frame(move || {
        render(&canvas, &world);
    });
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
