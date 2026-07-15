use std::{cell::RefCell, rc::Rc};

use crate::core::{primitives::point2d::Point2D, render::texture::Texture};

#[derive(Clone)]
pub struct Painting {
    pub texture: Rc<RefCell<Texture>>,
    pub top_left_corner: Point2D,
    pub bottom_right_corner: Point2D,
}

impl Painting {
    pub fn new(texture: Rc<RefCell<Texture>>, top_left_corner: Point2D) -> Self {
        let bottom = 1.0 - top_left_corner.y;

        let painting_height = bottom - top_left_corner.y;

        let painting_texture = texture.borrow();
        let painting_width_wallspace =
            painting_texture.width() as f64 * painting_height / painting_texture.height() as f64;
        drop(painting_texture);

        let right = painting_width_wallspace + top_left_corner.x;

        let bottom_right_corner = Point2D::new(right, bottom);

        Painting {
            texture,
            top_left_corner,
            bottom_right_corner,
        }
    }
}
