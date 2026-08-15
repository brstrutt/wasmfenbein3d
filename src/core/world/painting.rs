use std::rc::Rc;

use crate::core::{
    primitives::point2d::Point2D,
    render::{texel_provider::TexelProvider, texture::Texture},
    world::wall::WALL_HEIGHT,
};

#[derive(Clone)]
pub struct Painting {
    pub texture: Rc<Texture>,
    pub top_left_corner: Point2D,
    pub bottom_right_corner: Point2D,
    pub width: f64,
    pub height: f64,
    pub on_click: Option<fn()>,
}

impl Painting {
    pub fn new_to_scale(
        texture: Rc<Texture>,
        top_left_corner: Point2D,
        on_click: Option<fn()>,
    ) -> Self {
        let bottom = WALL_HEIGHT - top_left_corner.y;

        let painting_height = bottom - top_left_corner.y;

        let painting_width_wallspace =
            texture.width() as f64 * painting_height / texture.height() as f64;

        let right = painting_width_wallspace + top_left_corner.x;

        let bottom_right_corner = Point2D::new(right, bottom);

        Painting::new(texture, top_left_corner, bottom_right_corner, on_click)
    }

    pub fn new(
        texture: Rc<Texture>,
        top_left_corner: Point2D,
        bottom_right_corner: Point2D,
        on_click: Option<fn()>,
    ) -> Self {
        let size = bottom_right_corner - top_left_corner;
        Painting {
            texture,
            top_left_corner,
            bottom_right_corner,
            width: size.x,
            height: size.y,
            on_click,
        }
    }
}
