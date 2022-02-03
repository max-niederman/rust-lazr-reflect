use crate::{direction::*, vector::Vector};
use std::ops::{Index, IndexMut};
use tiny_skia::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Mirror { normal: Direction },
    Empty,
}

impl Tile {
    pub fn render(&self, pixmap: &mut Pixmap, x: f32, y: f32, size: f32) {
        match self {
            Tile::Mirror { normal } => {
                let center = Point {
                    x: x + size / 2.0,
                    y: y + size / 2.0,
                };
                let radius = size / 2.0 - 8.0;
                let angle = normal.angle_to(Direction::East) + Angle::RIGHT;

                let mut pb = PathBuilder::new();
                pb.move_to(
                    center.x + angle.cos() * radius,
                    center.y + angle.sin() * radius,
                );
                pb.line_to(
                    center.x - angle.cos() * radius,
                    center.y - angle.sin() * radius,
                );
                let path = pb.finish().unwrap();

                let mut paint = Paint::default();
                paint.set_color(Color::WHITE);
                paint.anti_alias = true;

                let mut stroke = Stroke::default();
                stroke.width = size / 5.0;
                stroke.line_cap = LineCap::Square;

                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
            Tile::Empty => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tilemap {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

impl Tilemap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            tiles: vec![Tile::Empty; width as usize * height as usize],
            width,
            height,
        }
    }

    pub fn render(&self, pixmap: &mut Pixmap) {
        let tile_size = pixmap.width() / self.width;
        for (i, tile) in self.tiles.iter().enumerate() {
            tile.render(
                pixmap,
                (i as u32 % self.width * tile_size) as f32,
                (i as u32 / self.width * tile_size) as f32,
                tile_size as f32,
            );
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn len(&self) -> u32 {
        self.width * self.height
    }
}

impl Index<u32> for Tilemap {
    type Output = Tile;

    fn index(&self, index: u32) -> &Self::Output {
        &self.tiles[index as usize]
    }
}

impl IndexMut<u32> for Tilemap {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.tiles[index as usize]
    }
}

impl Index<Vector<u32, 2>> for Tilemap {
    type Output = Tile;

    fn index(&self, index: Vector<u32, 2>) -> &Self::Output {
        &self.tiles[(index[1] * self.width + index[0]) as usize]
    }
}
