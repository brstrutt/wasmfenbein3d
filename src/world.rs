use wasmfenbein3d::core::{
    primitives::point2d::Point2D,
    render::textures::Textures,
    world::{wall::Wall, walls::walls_from_point_path},
};

pub fn load_walls(textures: &Textures) -> Vec<Wall> {
    let mut result = Vec::<Wall>::new();
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-5.0, -3.0),
            Point2D::new(-5.0, 5.0),
            Point2D::new(-1.0, 5.0),
        ],
        &textures.wall_wood,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-1.0, 5.0),
            Point2D::new(-1.0, 20.0),
            Point2D::new(8.0, 20.0),
        ],
        &textures.wall_stone,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(8.0, 20.0),
            Point2D::new(8.0, 23.0),
            Point2D::new(15.0, 23.0),
            Point2D::new(15.0, 15.0),
            Point2D::new(8.0, 15.0),
            Point2D::new(8.0, 18.0),
        ],
        &textures.wall_wood,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(8.0, 18.0),
            Point2D::new(1.0, 18.0),
            Point2D::new(1.0, 5.0),
        ],
        &textures.wall_stone,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(1.0, 5.0),
            Point2D::new(5.0, 5.0),
            Point2D::new(5.0, -5.0),
            Point2D::new(-5.0, -5.0),
        ],
        &textures.wall_wood,
    ));
    result.append(&mut walls_from_point_path(
        &vec![Point2D::new(-5.0, -5.0), Point2D::new(-10.0, -5.0)],
        &textures.wall_stone,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-10.0, -5.0),
            Point2D::new(-10.0, -7.0),
            Point2D::new(-13.0, -7.0),
            Point2D::new(-13.0, -1.0),
            Point2D::new(-10.0, -1.0),
            Point2D::new(-10.0, -3.0),
        ],
        &textures.wall_wood,
    ));
    result.append(&mut walls_from_point_path(
        &vec![Point2D::new(-10.0, -3.0), Point2D::new(-5.0, -3.0)],
        &textures.wall_stone,
    ));
    result
}
