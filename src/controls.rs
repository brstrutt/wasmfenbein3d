use std::{cell::RefCell, rc::Rc};
use web_sys::{KeyboardEvent};

use crate::{web, world::World};

pub fn setup(world: Rc<RefCell<World>>) {
    web::document::add_event_listener_with_callback(move |e: KeyboardEvent| {
        match e.key().as_str() {
            "a" | "A" => world.borrow_mut().camera.origin.x -= 10.0,
            "d" | "D" => world.borrow_mut().camera.origin.x += 10.0,
            &_ => return,
        }
    });
}