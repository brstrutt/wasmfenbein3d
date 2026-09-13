use crate::{
    controls::InputState,
    render::{camera::Camera, screen_buffer::ScreenBuffer},
};

pub mod stats;
pub mod textures;
pub mod world;
use world::*;

pub struct State<Screen: ScreenBuffer> {
    pub screen_buffer: Screen,
    pub world: world::World,
    pub camera: Camera,
    pub input: InputState,
    pub stats: stats::Stats,
}

impl<Screen: ScreenBuffer> State<Screen> {
    pub fn setup(screen_buffer: Screen, world: World) -> State<Screen> {
        State {
            camera: Camera::new(screen_buffer.width(), screen_buffer.height()),
            screen_buffer,
            world,
            input: InputState::setup(),
            stats: stats::Stats::new(),
        }
    }
}
