use wasmfenbein3d::{include_texture, state::textures::RawTexture};

// Tiling
pub const BIG_FLOOR: RawTexture = include_texture!("textures/", "big_floor", ".bmp");
pub const FLOOR: RawTexture = include_texture!("textures/", "floor", ".bmp");
pub const WALL_STONE: RawTexture = include_texture!("textures/", "wall_stone", ".bmp");
pub const WALL_WOOD: RawTexture = include_texture!("textures/", "wall_wood", ".bmp");

pub const TILING_TEXTURES: [&RawTexture; 4] = [&BIG_FLOOR, &FLOOR, &WALL_STONE, &WALL_WOOD];

// Non-tiling
pub const BLOOD_IN_THE_DARKNESS: RawTexture =
    include_texture!("textures/", "blood_in_the_darkness", ".jpg");
pub const BURPLESPUE_HALESCOURGE: RawTexture =
    include_texture!("textures/", "burplespue_halescourge", ".jpg");
pub const CASTLE_DRACHENFELS: RawTexture =
    include_texture!("textures/", "castle_drachenfels", ".jpg");
pub const INTO_THE_NEST: RawTexture = include_texture!("textures/", "into_the_nest", ".jpg");
pub const NOKIA_ART_JAM_3_HOUSE: RawTexture =
    include_texture!("textures/", "nokia_art_jam_3_house", ".bmp");
pub const NOKIA_ART_JAM_3_KEYBOARD_CAT: RawTexture =
    include_texture!("textures/", "nokia_art_jam_3_keyboard_cat", ".bmp");
pub const NOKIA_ART_JAM_3_WORMS: RawTexture =
    include_texture!("textures/", "nokia_art_jam_3_worms", ".bmp");
pub const RIGHTEOUS_STAND: RawTexture = include_texture!("textures/", "righteous_stand", ".jpg");
pub const TAALS_HORN_KEEP: RawTexture = include_texture!("textures/", "taals_horn_keep", ".jpg");
pub const UBERSREIK_FIVE: RawTexture = include_texture!("textures/", "ubersreik_five", ".bmp");
pub const VERMINTIDE_TAPESTRY: RawTexture =
    include_texture!("textures/", "vermintide_tapestry", ".bmp");

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
