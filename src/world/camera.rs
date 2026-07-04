use std::f64::consts::PI;

use crate::primitives::{point2d::Point2D, ray2d::Ray2D};

#[derive(Clone)]
pub struct Camera {
    pub ray: Ray2D,
}

const FOV_DEGREES: f64 = 90.0;
const FOV_RADIANS: f64 = PI * (FOV_DEGREES / 180.0);

impl Camera {
    pub fn dummy() -> Camera {
        Camera {
            ray: Ray2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 0.0, y: 1.0 }),
        }
    }

    pub fn ray_for_column(
        &self,
        column: usize,
        screen_height_pixels: usize,
        screen_width_pixels: usize,
    ) -> Ray2D {
        let fov_step_per_pixel = FOV_RADIANS / screen_width_pixels as f64;

        let screen_ratio = (screen_height_pixels / screen_width_pixels) as f64;
        let fov_screen_adjustment = fov_step_per_pixel * (screen_ratio - 1.0) * 0.5;

        let angle_step = fov_step_per_pixel - fov_screen_adjustment;
        let horizontal_pixel_coord: i64 = column as i64 - (screen_width_pixels as i64 / 2);

        let camera_direction_offset = angle_step * horizontal_pixel_coord as f64;

        self.ray.rotate(camera_direction_offset)
    }

    pub fn rotate(&self, angle_radians: f64) -> Camera {
        Camera {
            ray: self.ray.rotate(angle_radians),
        }
    }
}
