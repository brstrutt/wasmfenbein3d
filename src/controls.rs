use std::{cell::RefCell, rc::Rc};
use web_sys::{KeyboardEvent};

use crate::{web, world::World};

pub fn setup(world: Rc<RefCell<World>>) {
    web::document::add_event_listener_with_callback(move |e: KeyboardEvent| {
        match e.key().as_str() {
            "a" => world.borrow_mut().camera.position.x -= 10.0,
            "d" => world.borrow_mut().camera.position.x += 10.0,
            &_ => return,
        }
    });
}