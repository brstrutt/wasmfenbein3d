use std::f64::consts::PI;

use crate::primitives::{point2d::Point2D, ray2d::Ray2D};

#[derive(Clone)]
pub struct Camera {
    pub ray: Ray2D,
    screen_width: usize,
    screen_height: usize,
    screen_column_rays: Vec<Ray2D>,
}

const FOV_DEGREES: f64 = 90.0;
const FOV_RADIANS: f64 = PI * (FOV_DEGREES / 180.0);

impl Camera {
    pub fn new(screen_width: usize, screen_height: usize) -> Camera {
        let central_ray = Ray2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 0.0, y: 1.0 });
        Camera {
            ray: central_ray,
            screen_width,
            screen_height,
            screen_column_rays: Camera::calculate_all_column_rays(
                central_ray,
                screen_width,
                screen_height,
            ),
        }
    }

    pub fn refresh_screen_rays(&mut self) {
        self.screen_column_rays =
            Camera::calculate_all_column_rays(self.ray, self.screen_width, self.screen_height);
    }

    pub fn ray_for_column(&self, column: usize) -> Ray2D {
        let column = column % self.screen_width;
        self.screen_column_rays[column]
    }

    pub fn rotate(&self, angle_radians: f64) -> Camera {
        Camera {
            ray: self.ray.rotate(angle_radians),
            screen_width: self.screen_width,
            screen_height: self.screen_height,
            screen_column_rays: self.screen_column_rays.clone(),
        }
    }

    fn calculate_all_column_rays(
        central_ray: Ray2D,
        screen_width_pixels: usize,
        screen_height_pixels: usize,
    ) -> Vec<Ray2D> {
        let mut rays = vec![central_ray; screen_width_pixels];

        let fov_step_per_pixel = FOV_RADIANS / screen_width_pixels as f64;

        let screen_ratio = (screen_height_pixels / screen_width_pixels) as f64;
        let fov_screen_adjustment = fov_step_per_pixel * (screen_ratio - 1.0) * 0.5;

        let angle_step = fov_step_per_pixel - fov_screen_adjustment;

        for column in 0..screen_width_pixels {
            let horizontal_pixel_coord: i64 = column as i64 - (screen_width_pixels as i64 / 2);
            rays[column] = rays[column].rotate(angle_step * horizontal_pixel_coord as f64);
        }
        rays
    }
}
