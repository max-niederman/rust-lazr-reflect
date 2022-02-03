use crate::{direction::Direction, vector::Vector};
use tiny_skia::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Vector<f32, 2>,
    direction: Direction,
}

impl Ray {
    pub fn new(origin: Vector<f32, 2>, direction: Direction) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Vector<f32, 2> {
        self.origin
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn point(&self, distance: f32) -> Vector<f32, 2> {
        self.origin + Vector::in_direction(self.direction, distance)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Laser {
    start: f32,
    length: f32,

    // First line segment of the laser.
    tail: Ray,
    // Middle line segments of the laser.
    body: heapless::Vec<Ray, 32>,
    // Final line segment of the laser.
    head: Ray,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Collision {
    /// A collision which does not result in a discontinuity in the laser.
    /// E.g. a reflection or refraction.
    Continuous { new: Ray },
    /// A collision which _does_ result in a discontinuity in the laser.
    /// E.g. a portal.
    Discontinuous { new: Ray },
    /// A collision which does not affect the path of the laser.
    /// This can also be thought of as the lack of a collision.
    Null,
}

pub trait LaserCollide {
    /// Test if a [`Laser`] collides with the value and if it does, returns the new ray.
    fn collide(&self, laser: &Laser) -> Collision;
}
