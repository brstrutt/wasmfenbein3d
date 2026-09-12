use serde::Deserialize;

use super::wall::Wall;
use crate::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    state::{textures::TextureLibrary, world::painting::Painting},
};

#[derive(Deserialize)]
struct JsonWorld {
    walls: Vec<JsonWall>,
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

pub fn load_walls_from_json(textures: &TextureLibrary, json: &str) -> Vec<Wall> {
    let world: JsonWorld = serde_json::from_str(json).expect("JSON was not well-formatted");
    let mut walls = Vec::<Wall>::new();
    for wall in world.walls {
        walls.push(wall.to_wall(textures));
    }
    walls
}
