use crate::{controls::InputState, web, world::World};


pub struct GameState {
    pub world: World,
    pub input: InputState,
    pub last_frame_time_ms: f64,
}

impl GameState {
    pub fn setup() -> GameState {
        GameState {
            world: World::dummy(),
            input: InputState::setup(),
            last_frame_time_ms: web::window::now_in_ms(),
        }
    }
}