use std::{cell::RefCell, rc::Rc};
use web_sys::{KeyboardEvent};

use crate::{web, world::World};

pub fn setup(world: Rc<RefCell<World>>) {
    web::document::add_event_listener_with_callback(move |e: KeyboardEvent| {
        let mut speed = 0.1;
        if e.shift_key() {
            speed = speed * 10.0;
        }
        match e.key().as_str() {
            "a" | "A" => world.borrow_mut().camera.origin.x += speed,
            "d" | "D" => world.borrow_mut().camera.origin.x -= speed,
            "w" | "W" => world.borrow_mut().camera.origin.y += speed,
            "s" | "S" => world.borrow_mut().camera.origin.y -= speed,
            &_ => return,
        }
    });
}