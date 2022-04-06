use lmaths::*;
use crate::Shape;

#[allow(dead_code)]
pub struct Circle {
    pub position:Vector2,
    pub radius:f64
}

#[allow(dead_code)]
impl Circle {

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