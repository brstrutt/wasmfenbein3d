use crate::{main_canvas::MainCanvas, render::rgb::RGB, world::{World, camera::Camera}};

pub mod screen;
pub mod rgb;


pub fn render(canvas: &MainCanvas, world: &World, camera: &Camera) {
    screen::clear(canvas);


    for x in 0..=canvas.element.width() {
        let ray = camera.ray_for_column(x);
        let wall_distance = world.dist_to_wall(&ray);

        if wall_distance.is_some() {
            let distance = wall_distance.unwrap().round() as u32;
            let height = canvas.element.height() * 10 / (distance + 100);
            screen::render_column(canvas, x, height, &RGB {red: 30 * 100 / distance, green: 150 * 100 / distance, blue: 30 * 100 / distance});
        }
    }
}
