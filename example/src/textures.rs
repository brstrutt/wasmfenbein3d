use wasmfenbein3d::{include_texture, state::textures::RawTexture};

// Tiling
pub const BIG_FLOOR: RawTexture = include_texture!("big_floor");
pub const FLOOR: RawTexture = include_texture!("floor");
pub const WALL_STONE: RawTexture = include_texture!("wall_stone");
pub const WALL_WOOD: RawTexture = include_texture!("wall_wood");

pub const TILING_TEXTURES: [&RawTexture; 4] = [&BIG_FLOOR, &FLOOR, &WALL_STONE, &WALL_WOOD];

// Non-tiling
pub const BLOOD_IN_THE_DARKNESS: RawTexture = include_texture!("blood_in_the_darkness");
pub const BURPLESPUE_HALESCOURGE: RawTexture = include_texture!("burplespue_halescourge");
pub const CASTLE_DRACHENFELS: RawTexture = include_texture!("castle_drachenfels");
pub const INTO_THE_NEST: RawTexture = include_texture!("into_the_nest");
pub const NOKIA_ART_JAM_3_HOUSE: RawTexture = include_texture!("nokia_art_jam_3_house");
pub const NOKIA_ART_JAM_3_KEYBOARD_CAT: RawTexture =
    include_texture!("nokia_art_jam_3_keyboard_cat");
pub const NOKIA_ART_JAM_3_WORMS: RawTexture = include_texture!("nokia_art_jam_3_worms");
pub const RIGHTEOUS_STAND: RawTexture = include_texture!("righteous_stand");
pub const TAALS_HORN_KEEP: RawTexture = include_texture!("taals_horn_keep");
pub const UBERSREIK_FIVE: RawTexture = include_texture!("ubersreik_five");
pub const VERMINTIDE_TAPESTRY: RawTexture = include_texture!("vermintide_tapestry");

pub const TEXTURES: [&RawTexture; 11] = [
    &BLOOD_IN_THE_DARKNESS,
    &BURPLESPUE_HALESCOURGE,
    &CASTLE_DRACHENFELS,
    &INTO_THE_NEST,
    &NOKIA_ART_JAM_3_HOUSE,
    &NOKIA_ART_JAM_3_KEYBOARD_CAT,
    &NOKIA_ART_JAM_3_WORMS,
    &RIGHTEOUS_STAND,
    &TAALS_HORN_KEEP,
    &UBERSREIK_FIVE,
    &VERMINTIDE_TAPESTRY,
];
