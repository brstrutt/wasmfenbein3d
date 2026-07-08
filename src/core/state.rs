use crate::core::{controls::InputState, web, world::World};

pub struct GameState {
    pub world: World,
    pub input: InputState,
    pub last_frame_time_ms: f64,
    pub last_time_between_frames_ms: f64,
    pub last_time_to_render_one_frame_ms: f64,
}

impl GameState {
    pub fn setup(screen_width: usize, screen_height: usize) -> GameState {
        GameState {
            world: World::new(screen_width, screen_height),
            input: InputState::setup(),
            last_frame_time_ms: web::window::now_in_ms(),
            last_time_between_frames_ms: 0.0,
            last_time_to_render_one_frame_ms: 0.0,
        }
    }
}
