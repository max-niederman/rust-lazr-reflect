use tiny_skia::*;

use crate::direction::*;

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
                let radius = size / 2.0;
                let angle = normal.angle_to(&Direction::East) + Angle::RIGHT;

                let mut pb = PathBuilder::new();
                pb.move_to(
                    center.x + angle.cos() * radius,
                    center.y + angle.sin() * radius,
                );
                pb.line_to(
                    center.x - angle.sin() * radius,
                    center.y - angle.cos() * radius,
                );
                let path = pb.finish().unwrap();

                let mut paint = Paint::default();
                paint.set_color(Color::BLACK);
                pixmap.fill_rect(
                    Rect::from_xywh(x, y, size, size).unwrap(),
                    &paint,
                    Transform::identity(),
                    None,
                );
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
