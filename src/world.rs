use wasmfenbein3d::core::{
    primitives::point2d::Point2D,
    render::rgb_palette::RgbPalette,
    world::{painting::Painting, wall::Wall, walls::walls_from_point_path},
};

use crate::textures;

pub fn load_walls(palette: &mut RgbPalette) -> Vec<Wall> {
    let wood_wall_texture = textures::wall_wood::load_texture(palette);
    let stone_wall_texture = textures::wall_stone::load_texture(palette);
    let vermintide_tapestry = textures::vermintide_tapestry::load_texture(palette);
    let default_painting = Painting {
        texture: vermintide_tapestry,
    };

    let mut result = Vec::<Wall>::new();
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-5.0, -3.0),
            Point2D::new(-5.0, 5.0),
            Point2D::new(-1.0, 5.0),
        ],
        &wood_wall_texture,
        default_painting.clone(),
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-1.0, 5.0),
            Point2D::new(-1.0, 20.0),
            Point2D::new(8.0, 20.0),
        ],
        &stone_wall_texture,
        default_painting.clone(),
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
        &wood_wall_texture,
        default_painting.clone(),
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(8.0, 18.0),
            Point2D::new(1.0, 18.0),
            Point2D::new(1.0, 5.0),
        ],
        &stone_wall_texture,
        default_painting.clone(),
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(1.0, 5.0),
            Point2D::new(5.0, 5.0),
            Point2D::new(5.0, -5.0),
            Point2D::new(-5.0, -5.0),
        ],
        &wood_wall_texture,
        default_painting.clone(),
    ));
    result.append(&mut walls_from_point_path(
        &vec![Point2D::new(-5.0, -5.0), Point2D::new(-10.0, -5.0)],
        &stone_wall_texture,
        default_painting.clone(),
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
        &wood_wall_texture,
        default_painting.clone(),
    ));
    result.append(&mut walls_from_point_path(
        &vec![Point2D::new(-10.0, -3.0), Point2D::new(-5.0, -3.0)],
        &stone_wall_texture,
        default_painting.clone(),
    ));
    result
}
