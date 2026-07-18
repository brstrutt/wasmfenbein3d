use wasmfenbein3d::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::rgb_palette::RgbPalette,
    world::{painting::Painting, wall::Wall, walls::walls_from_point_path},
};

use crate::textures;

pub fn load_walls(palette: &mut RgbPalette) -> Vec<Wall> {
    let wood_wall_texture = textures::wall_wood::load_texture(palette);
    let stone_wall_texture = textures::wall_stone::load_texture(palette);
    let vermintide_tapestry = textures::vermintide_tapestry::load_texture(palette);
    let nokia_jam_house = textures::nokia_art_jam_3_house::load_texture(palette);
    let nokia_jam_cat = textures::nokia_art_jam_3_keyboard_cat::load_texture(palette);
    let nokia_jam_worms = textures::nokia_art_jam_3_worms::load_texture(palette);

    let mut result = Vec::<Wall>::new();
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-1.0, 5.0),
            Point2D::new(-5.0, 5.0),
            Point2D::new(-5.0, -3.0),
        ],
        &wood_wall_texture,
        vec![],
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(8.0, 20.0),
            Point2D::new(-1.0, 20.0),
            Point2D::new(-1.0, 5.0),
        ],
        &stone_wall_texture,
        vec![],
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(8.0, 18.0),
            Point2D::new(8.0, 15.0),
            Point2D::new(15.0, 15.0),
        ],
        &wood_wall_texture,
        vec![],
    ));
    result.push(Wall::new(
        Line2D {
            start: Point2D::new(15.0, 15.0),
            end: Point2D::new(15.0, 23.0),
        },
        &wood_wall_texture,
        vec![Painting::new_to_scale(
            vermintide_tapestry,
            Point2D::new(3.5, 0.1),
        )],
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(15.0, 23.0),
            Point2D::new(8.0, 23.0),
            Point2D::new(8.0, 20.0),
        ],
        &wood_wall_texture,
        vec![],
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(1.0, 5.0),
            Point2D::new(1.0, 18.0),
            Point2D::new(8.0, 18.0),
        ],
        &stone_wall_texture,
        vec![],
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-5.0, -5.0),
            Point2D::new(5.0, -5.0),
            Point2D::new(5.0, 5.0),
            Point2D::new(1.0, 5.0),
        ],
        &wood_wall_texture,
        vec![],
    ));
    result.append(&mut walls_from_point_path(
        &vec![Point2D::new(-10.0, -5.0), Point2D::new(-5.0, -5.0)],
        &stone_wall_texture,
        vec![
            Painting::new_to_scale(nokia_jam_house, Point2D::new(0.5, 0.2)),
            Painting::new_to_scale(nokia_jam_cat, Point2D::new(1.75, 0.2)),
            Painting::new_to_scale(nokia_jam_worms, Point2D::new(3.0, 0.2)),
        ],
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-10.0, -3.0),
            Point2D::new(-10.0, -1.0),
            Point2D::new(-13.0, -1.0),
            Point2D::new(-13.0, -7.0),
            Point2D::new(-10.0, -7.0),
            Point2D::new(-10.0, -5.0),
        ],
        &wood_wall_texture,
        vec![],
    ));
    result.append(&mut walls_from_point_path(
        &vec![Point2D::new(-5.0, -3.0), Point2D::new(-10.0, -3.0)],
        &stone_wall_texture,
        vec![],
    ));
    result
}
