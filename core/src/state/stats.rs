pub struct Stats {
    pub render_frame: Timings,
    pub physics_frame: Timings,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            render_frame: Timings::new(),
            physics_frame: Timings::new(),
        }
    }
}

pub struct Timings {
    pub last_time_ms: f64,
    pub last_duration_ms: f64,
}

impl Timings {
    pub fn new() -> Self {
        Self {
            last_time_ms: 0.0,
            last_duration_ms: 0.0,
        }
    }
}
