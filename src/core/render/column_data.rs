use crate::core::{
    primitives::point2d::Point2D,
    world::{painting::Painting, wall::WALL_HEIGHT, walls::WallCollision},
};

pub struct ColumnData<'a> {
    pub wall_x_pos: f64,
    pub nearest_wall_intersection: &'a WallCollision<'a>,
    pub paintings: Vec<&'a Painting>,
    pub distance_from_camera: f64,
    pub height_pixels: f64,
}

impl<'a> ColumnData<'a> {
    pub fn init(
        nearest_wall_intersection: &'a WallCollision,
        camera_position: &Point2D,
        screen_height_f64: &f64,
    ) -> Self {
        let distance_from_camera =
            Point2D::dist(camera_position, &nearest_wall_intersection.intersection);

        let height_pixels = if distance_from_camera != 0.0 {
            WALL_HEIGHT * screen_height_f64 / distance_from_camera
        } else {
            0.0
        };

        let wall_x_pos = nearest_wall_intersection
            .wall
            .get_wall_space_x_position(&nearest_wall_intersection.intersection);
        let paintings = nearest_wall_intersection
            .wall
            .get_paintings_in_column(wall_x_pos);

        ColumnData {
            wall_x_pos,
            nearest_wall_intersection,
            paintings,
            distance_from_camera,
            height_pixels,
        }
    }
}
