use std::rc::Rc;

use crate::core::{
    controls::InputState,
    render::{camera::Camera, texel_provider::TexelProvider},
};

pub mod textures;
pub mod world;
use world::*;

pub struct GameState {
    pub world: world::World,
    pub camera: Camera,
    pub input: InputState,
    pub last_frame_time_ms: f64,
    pub last_time_between_frames_ms: f64,
    pub last_time_to_render_one_frame_ms: f64,
}

impl GameState {
    pub fn setup(
        screen_width: usize,
        screen_height: usize,
        walls: Vec<wall::Wall>,
        floor: &Rc<Box<dyn TexelProvider>>,
        ceiling: &Rc<Box<dyn TexelProvider>>,
    ) -> GameState {
        GameState {
            world: World::new(walls, floor, ceiling),
            camera: Camera::new(screen_width, screen_height),
            input: InputState::setup(),
            last_frame_time_ms: 0.0,
            last_time_between_frames_ms: 0.0,
            last_time_to_render_one_frame_ms: 0.0,
        }
    }
}
