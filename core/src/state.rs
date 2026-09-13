use crate::{
    controls::InputState,
    render::{camera::Camera, screen_buffer::ScreenBuffer},
};

pub mod textures;
pub mod world;
use world::*;

pub struct GameState {
    pub screen_buffer: Box<dyn ScreenBuffer>,
    pub world: world::World,
    pub camera: Camera,
    pub input: InputState,
    pub last_frame_time_ms: f64,
    pub last_time_between_frames_ms: f64,
    pub last_time_to_render_one_frame_ms: f64,
}

impl GameState {
    pub fn setup(screen_buffer: Box<dyn ScreenBuffer>, world: World) -> GameState {
        GameState {
            camera: Camera::new(screen_buffer.width(), screen_buffer.height()),
            screen_buffer,
            world,
            input: InputState::setup(),
            last_frame_time_ms: 0.0,
            last_time_between_frames_ms: 0.0,
            last_time_to_render_one_frame_ms: 0.0,
        }
    }
}
