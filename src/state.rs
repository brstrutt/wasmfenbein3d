use crate::{controls::InputState, world::World};


pub struct GameState {
    pub world: World,
    pub input: InputState,
}

impl GameState {
    pub fn setup() -> GameState {
        GameState {
            world: World::dummy(),
            input: InputState::setup(),
        }
    }
}