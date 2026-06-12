use crate::{primitives::{point2d::Point2D, ray2d::Ray2D}};

pub type Camera = Ray2D;

impl Camera {
    pub fn dummy() -> Camera {
        Ray2D::new(
            Point2D { x: 0.0, y: 0.0 },
            Point2D { x: 0.0, y: 1.0 }
        )
    }

    pub fn ray_for_column(&self, column: u32, screen_height_pixels: u32, screen_width_pixels: u32) -> Ray2D {
        const FOV_DEGRESS: f64 = std::f32::consts::PI as f64 / 4.0; // 45 degrees in radians

        let angle_step = FOV_DEGRESS / screen_height_pixels as f64;
        let horizontal_pixel_coord: i64 = column as i64 - (screen_width_pixels as i64 / 2);

        let camera_direction_offset = angle_step * horizontal_pixel_coord as f64;

        self.rotate(camera_direction_offset)
    }
}