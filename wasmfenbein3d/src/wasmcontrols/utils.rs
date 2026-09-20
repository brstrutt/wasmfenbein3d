use fenbein3d::controls::Direction;

pub fn key_to_direction(key: &str) -> Option<Direction> {
    match key {
        "a" | "A" => Some(Direction::Left),
        "d" | "D" => Some(Direction::Right),
        "w" | "W" => Some(Direction::Forward),
        "s" | "S" => Some(Direction::Backward),
        &_ => None,
    }
}
