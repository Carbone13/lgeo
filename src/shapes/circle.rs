use std::fmt::{Display, Formatter};
use lmaths::*;
use crate::Shape;

#[allow(dead_code)]
#[derive(Clone, Copy, Default, PartialEq, Debug)]
/// A circle represented by a position and a radius
pub struct Circle {
    pub position:Vector2,
    pub radius:f64
}

#[allow(dead_code)]
impl Circle {
    /// Create a new circle at the specified location, with a certain radius
    pub fn new (position:Vector2, radius:f64) -> Self {
        Self { position, radius }
    }
}

impl Shape for Circle {
    fn position(&self) -> Vector2 {
        self.position
    }

    fn support_point(&self, direction: Vector2) -> Vector2 {
        (direction.normalized() * self.radius) + self.position()
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle pos ({0}), radius: {1}", self.position, self.radius)
    }
}