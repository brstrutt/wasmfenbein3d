use wasmfenbein3d::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::rgb_palette::RgbPalette,
    world::{painting::Painting, wall::Wall, walls::walls_from_point_path},
};

use crate::textures;

pub mod entity_ids {
    pub const DUMMY_ID: &str = "no_on_click_behaviour";
    pub const NOKIA_JAM_HOUSE_ID: &str = "nokia_jam_house";
    pub const NOKIA_JAM_CAT_ID: &str = "nokia_jam_cat";
    pub const NOKIA_JAM_WORMS_ID: &str = "nokia_jam_worms";
    pub const UBERSREIK_FIVE_ID: &str = "ubersreik_five";
    pub const VERMINTIDE_TAPESTRY_ID: &str = "vermintide_tapestry";
}

use entity_ids::*;

pub fn load_walls(palette: &mut RgbPalette) -> Vec<Wall> {
    let wood_wall_texture = textures::wall_wood::load_texture(palette);
    let stone_wall_texture = textures::wall_stone::load_texture(palette);
    let vermintide_tapestry = textures::vermintide_tapestry::load_texture(palette);
    let nokia_jam_house = textures::nokia_art_jam_3_house::load_texture(palette);
    let nokia_jam_cat = textures::nokia_art_jam_3_keyboard_cat::load_texture(palette);
    let nokia_jam_worms = textures::nokia_art_jam_3_worms::load_texture(palette);
    let ubersreik_five = textures::ubersreik_five::load_texture(palette);
    let blood_in_the_darkness = textures::blood_in_the_darkness::load_texture(palette);
    let burplespue_halescourge = textures::burplespue_halescourge::load_texture(palette);
    let castle_drachenfels = textures::castle_drachenfels::load_texture(palette);
    let into_the_nest = textures::into_the_nest::load_texture(palette);
    let righteous_stand = textures::righteous_stand::load_texture(palette);
    let taals_horn_keep = textures::taals_horn_keep::load_texture(palette);

    let mut result = Vec::<Wall>::new();
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-1.0, 5.0),
            Point2D::new(-5.0, 5.0),
            Point2D::new(-5.0, -3.0),
        ],
        &wood_wall_texture,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(8.0, 20.0),
            Point2D::new(-1.0, 20.0),
            Point2D::new(-1.0, 5.0),
        ],
        &stone_wall_texture,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(8.0, 18.0),
            Point2D::new(8.0, 15.0),
            Point2D::new(15.0, 15.0),
        ],
        &wood_wall_texture,
    ));
    result.push(Wall::new(
        Line2D {
            start: Point2D::new(15.0, 15.0),
            end: Point2D::new(15.0, 23.0),
        },
        &wood_wall_texture,
        vec![
            Painting::new_to_scale(
                VERMINTIDE_TAPESTRY_ID,
                vermintide_tapestry,
                Point2D::new(2.85, 0.1),
            ),
            Painting::new(
                DUMMY_ID,
                blood_in_the_darkness,
                Point2D::new(2.50, 0.5),
                Point2D::new(2.80, 0.8),
            ),
            Painting::new(
                DUMMY_ID,
                burplespue_halescourge,
                Point2D::new(2.50, 1.3),
                Point2D::new(2.80, 1.6),
            ),
            Painting::new(
                DUMMY_ID,
                castle_drachenfels,
                Point2D::new(2.50, 0.9),
                Point2D::new(2.80, 1.2),
            ),
            Painting::new(
                DUMMY_ID,
                into_the_nest,
                Point2D::new(5.40, 0.5),
                Point2D::new(5.70, 0.8),
            ),
            Painting::new(
                DUMMY_ID,
                righteous_stand,
                Point2D::new(5.40, 1.3),
                Point2D::new(5.70, 1.6),
            ),
            Painting::new(
                DUMMY_ID,
                taals_horn_keep,
                Point2D::new(5.40, 0.9),
                Point2D::new(5.70, 1.2),
            ),
        ],
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(15.0, 23.0),
            Point2D::new(8.0, 23.0),
            Point2D::new(8.0, 20.0),
        ],
        &wood_wall_texture,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(1.0, 5.0),
            Point2D::new(1.0, 18.0),
            Point2D::new(8.0, 18.0),
        ],
        &stone_wall_texture,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-5.0, -5.0),
            Point2D::new(5.0, -5.0),
            Point2D::new(5.0, 5.0),
            Point2D::new(1.0, 5.0),
        ],
        &wood_wall_texture,
    ));
    result.push(Wall::new(
        Line2D {
            start: Point2D::new(-10.0, -5.0),
            end: Point2D::new(-5.0, -5.0),
        },
        &stone_wall_texture,
        vec![
            Painting::new_to_scale(NOKIA_JAM_HOUSE_ID, nokia_jam_house, Point2D::new(0.4, 0.6)),
            Painting::new_to_scale(NOKIA_JAM_CAT_ID, nokia_jam_cat, Point2D::new(2.0, 0.6)),
            Painting::new_to_scale(NOKIA_JAM_WORMS_ID, nokia_jam_worms, Point2D::new(3.5, 0.6)),
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
    ));
    result.push(Wall::new(
        Line2D {
            start: Point2D::new(-5.0, -3.0),
            end: Point2D::new(-10.0, -3.0),
        },
        &stone_wall_texture,
        vec![Painting::new_to_scale(
            UBERSREIK_FIVE_ID,
            ubersreik_five,
            Point2D::new(3.0, 0.2),
        )],
    ));
    result
}
