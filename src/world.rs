use wasmfenbein3d::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    state::{
        textures::TextureLibrary,
        world::{painting::Painting, wall::Wall, walls::walls_from_point_path},
    },
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

pub fn load_walls(textures: &TextureLibrary) -> Vec<Wall> {
    let mut result = Vec::<Wall>::new();
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-1.0, 5.0),
            Point2D::new(-5.0, 5.0),
            Point2D::new(-5.0, -3.0),
        ],
        textures.get(textures::WALL_WOOD.id),
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(8.0, 20.0),
            Point2D::new(-1.0, 20.0),
            Point2D::new(-1.0, 5.0),
        ],
        textures.get(textures::WALL_STONE.id),
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(8.0, 18.0),
            Point2D::new(8.0, 15.0),
            Point2D::new(15.0, 15.0),
        ],
        textures.get(textures::WALL_WOOD.id),
    ));
    result.push(Wall::new(
        Line2D {
            start: Point2D::new(15.0, 15.0),
            end: Point2D::new(15.0, 23.0),
        },
        textures.get(textures::WALL_WOOD.id),
        vec![
            Painting::new_to_scale(
                VERMINTIDE_TAPESTRY_ID,
                textures.get(textures::VERMINTIDE_TAPESTRY.id),
                Point2D::new(2.85, 0.1),
            ),
            Painting::new(
                DUMMY_ID,
                textures.get(textures::BLOOD_IN_THE_DARKNESS.id),
                Point2D::new(2.50, 0.5),
                Point2D::new(2.80, 0.8),
            ),
            Painting::new(
                DUMMY_ID,
                textures.get(textures::BURPLESPUE_HALESCOURGE.id),
                Point2D::new(2.50, 1.3),
                Point2D::new(2.80, 1.6),
            ),
            Painting::new(
                DUMMY_ID,
                textures.get(textures::CASTLE_DRACHENFELS.id),
                Point2D::new(2.50, 0.9),
                Point2D::new(2.80, 1.2),
            ),
            Painting::new(
                DUMMY_ID,
                textures.get(textures::INTO_THE_NEST.id),
                Point2D::new(5.40, 0.5),
                Point2D::new(5.70, 0.8),
            ),
            Painting::new(
                DUMMY_ID,
                textures.get(textures::INTO_THE_NEST.id),
                Point2D::new(5.40, 1.3),
                Point2D::new(5.70, 1.6),
            ),
            Painting::new(
                DUMMY_ID,
                textures.get(textures::INTO_THE_NEST.id),
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
        textures.get(textures::WALL_WOOD.id),
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(1.0, 5.0),
            Point2D::new(1.0, 18.0),
            Point2D::new(8.0, 18.0),
        ],
        textures.get(textures::WALL_STONE.id),
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-5.0, -5.0),
            Point2D::new(5.0, -5.0),
            Point2D::new(5.0, 5.0),
            Point2D::new(1.0, 5.0),
        ],
        textures.get(textures::WALL_WOOD.id),
    ));
    result.push(Wall::new(
        Line2D {
            start: Point2D::new(-10.0, -5.0),
            end: Point2D::new(-5.0, -5.0),
        },
        textures.get(textures::WALL_WOOD.id),
        vec![
            Painting::new_to_scale(
                NOKIA_JAM_HOUSE_ID,
                textures.get(textures::NOKIA_ART_JAM_3_HOUSE.id),
                Point2D::new(0.4, 0.6),
            ),
            Painting::new_to_scale(
                NOKIA_JAM_CAT_ID,
                textures.get(textures::NOKIA_ART_JAM_3_KEYBOARD_CAT.id),
                Point2D::new(2.0, 0.6),
            ),
            Painting::new_to_scale(
                NOKIA_JAM_WORMS_ID,
                textures.get(textures::NOKIA_ART_JAM_3_WORMS.id),
                Point2D::new(3.5, 0.6),
            ),
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
        textures.get(textures::WALL_WOOD.id),
    ));
    result.push(Wall::new(
        Line2D {
            start: Point2D::new(-5.0, -3.0),
            end: Point2D::new(-10.0, -3.0),
        },
        textures.get(textures::WALL_STONE.id),
        vec![Painting::new_to_scale(
            UBERSREIK_FIVE_ID,
            textures.get(textures::UBERSREIK_FIVE.id),
            Point2D::new(3.0, 0.2),
        )],
    ));
    result
}
