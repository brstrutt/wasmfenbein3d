use serde::Deserialize;

use super::wall::Wall;
use crate::{
    primitives::{line2d::Line2D, point2d::Point2D},
    state::{
        textures::TextureLibrary,
        world::{World, painting::Painting},
    },
};

#[derive(Deserialize)]
pub struct JsonWorld {
    floor_texture_id: String,
    ceiling_texture_id: String,
    walls: Vec<JsonWall>,
}

impl JsonWorld {
    pub fn to_world(&self, textures: &TextureLibrary) -> World {
        World::new(
            self.walls
                .iter()
                .map(|wall| wall.to_wall(textures))
                .collect(),
            textures.get(&self.floor_texture_id),
            textures.get(&self.ceiling_texture_id),
        )
    }
}

#[derive(Deserialize)]
struct JsonWall {
    start: Point2D,
    end: Point2D,
    texture_id: String,
    paintings: Option<Vec<JsonPainting>>,
}

impl JsonWall {
    pub fn to_wall(&self, textures: &TextureLibrary) -> Wall {
        let mut loaded_paintings = vec![];
        if let Some(paintings) = &self.paintings {
            for painting in paintings {
                loaded_paintings.push(painting.to_painting(textures))
            }
        }
        Wall::new(
            Line2D {
                start: self.start,
                end: self.end,
            },
            textures.get(self.texture_id.as_str()),
            loaded_paintings,
        )
    }
}

#[derive(Deserialize)]
struct JsonPainting {
    id: String,
    top_left: Point2D,
    bottom_right: Option<Point2D>,
}

impl JsonPainting {
    pub fn to_painting(&self, textures: &TextureLibrary) -> Painting {
        if let Some(bottom_right) = self.bottom_right {
            Painting::new(
                self.id.as_str(),
                textures.get(self.id.as_str()),
                self.top_left,
                bottom_right,
            )
        } else {
            Painting::new_to_scale(
                self.id.as_str(),
                textures.get(self.id.as_str()),
                self.top_left,
            )
        }
    }
}
