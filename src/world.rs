use wasmfenbein3d::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::rgb_palette::RgbPalette,
    world::{painting::Painting, wall::Wall, walls::walls_from_point_path},
};
use web_sys::console::log;

use crate::{
    textures,
    web::{self, access::popup_page},
};

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
        vec![
            Painting::new_to_scale(
                vermintide_tapestry,
                Point2D::new(2.85, 0.1),
                Some(|| log::info!("Clicked on the tapestry!")),
            ),
            Painting::new(
                blood_in_the_darkness,
                Point2D::new(2.50, 0.5),
                Point2D::new(2.80, 0.8),
                None,
            ),
            Painting::new(
                burplespue_halescourge,
                Point2D::new(2.50, 1.3),
                Point2D::new(2.80, 1.6),
                None,
            ),
            Painting::new(
                castle_drachenfels,
                Point2D::new(2.50, 0.9),
                Point2D::new(2.80, 1.2),
                None,
            ),
            Painting::new(
                into_the_nest,
                Point2D::new(5.40, 0.5),
                Point2D::new(5.70, 0.8),
                None,
            ),
            Painting::new(
                righteous_stand,
                Point2D::new(5.40, 1.3),
                Point2D::new(5.70, 1.6),
                None,
            ),
            Painting::new(
                taals_horn_keep,
                Point2D::new(5.40, 0.9),
                Point2D::new(5.70, 1.2),
                None,
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
            Painting::new_to_scale(
                nokia_jam_house,
                Point2D::new(0.4, 0.6),
                Some(|| log::info!("House is thinking it's not Lupus!")),
            ),
            Painting::new_to_scale(
                nokia_jam_cat,
                Point2D::new(2.0, 0.6),
                Some(|| log::info!("Look at that cat GO!")),
            ),
            Painting::new_to_scale(
                nokia_jam_worms,
                Point2D::new(3.5, 0.6),
                Some(|| log::info!("Damn these worms are ANGRY!")),
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
        &wood_wall_texture,
        vec![],
    ));
    result.append(&mut walls_from_point_path(
        &vec![Point2D::new(-5.0, -3.0), Point2D::new(-10.0, -3.0)],
        &stone_wall_texture,
        vec![Painting::new_to_scale(
            ubersreik_five,
            Point2D::new(3.0, 0.2),
            Some(|| {
                let popup_page = web::access::popup_page();
                popup_page.set_hidden(false);
            }),
        )],
    ));
    result
}
