use std::{cell::RefCell, rc::Rc};
use web_sys::{KeyboardEvent};

use crate::{web, world::World};

pub fn setup(world: Rc<RefCell<World>>) {
    web::document::add_event_listener_with_callback(move |e: KeyboardEvent| {
        let mut speed = 0.1;
        if e.shift_key() {
            speed = speed * 10.0;
        }

        let mut world = world.borrow_mut();
        match e.key().as_str() {
            "a" | "A" => world.camera.origin = world.camera.origin - world.camera.rotate(std::f32::consts::PI as f64 / 2.0).direction,
            "d" | "D" => world.camera.origin = world.camera.origin + world.camera.rotate(std::f32::consts::PI as f64 / 2.0).direction,
            "w" | "W" => world.camera.origin = world.camera.origin + world.camera.direction,
            "s" | "S" => world.camera.origin = world.camera.origin - world.camera.direction,
            "ArrowRight" => world.camera = world.camera.rotate(0.1),
            "ArrowLeft" => world.camera = world.camera.rotate(-0.1),
            &_ => return,
        }
    });
}